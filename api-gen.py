#!/usr/bin/env python3
"""
Rust Bindings Generator for Markdown

Automatically generates markdown documentation for public Rust functions and structs.
Usage: python generate_bindings.py <path_to_rust_file_or_directory> [-o output.md]
"""

import os
import re
import sys
import argparse
from pathlib import Path
from dataclasses import dataclass, field
from typing import List, Optional


# Files to exclude from processing
EXCLUDED_FILES = {"rust_ffi.rs"}


@dataclass
class DocItem:
    kind: str  # "function" or "struct"
    name: str
    signature: str
    comments: str
    file_name: str
    line_number: int


def extract_doc_comments(lines: List[str], end_index: int) -> str:
    """Extract doc comments (///) and block doc comments (/** */) preceding a line index."""
    comments = []
    i = end_index - 1
    
    # Skip back over attributes like #[derive(...)], #[inline], etc.
    while i >= 0:
        stripped = lines[i].strip()
        if stripped.startswith('#[') or stripped.startswith('#!['):
            i -= 1
        else:
            break
    
    # Collect doc comments going backwards
    while i >= 0:
        stripped = lines[i].strip()
        if stripped.startswith('///'):
            comments.insert(0, stripped[3:].strip())
            i -= 1
        elif stripped.startswith('//!'):
            comments.insert(0, stripped[3:].strip())
            i -= 1
        elif stripped.startswith('//') and not stripped.startswith('///'):
            # Regular comment - stop here (not a doc comment)
            break
        elif stripped == '':
            # Allow blank line between doc comments, but check if there's more above
            j = i - 1
            while j >= 0 and lines[j].strip() == '':
                j -= 1
            if j >= 0 and (lines[j].strip().startswith('///') or lines[j].strip().startswith('//!')):
                i -= 1
            else:
                break
        else:
            break
    
    return '\n'.join(comments) if comments else ""


def extract_block_doc_comment(lines: List[str], end_index: int) -> Optional[str]:
    """Check for /** ... */ block doc comments ending right before end_index."""
    # Search backwards for /** ... */
    i = end_index - 1
    while i >= 0:
        stripped = lines[i].strip()
        if stripped.startswith('#['):
            i -= 1
            continue
        break
    
    # Look for */ 
    if i < 0:
        return None
    
    # Try to find a /* ... */ block
    j = i
    while j >= 0:
        stripped = lines[j].strip()
        if '*/' in stripped:
            # Found end of block comment
            # Now find the start
            k = j
            while k >= 0:
                if '/**' in lines[k]:
                    # Extract content
                    content_lines = []
                    for idx in range(k, j + 1):
                        line = lines[idx]
                        if idx == k:
                            pos = line.find('/**')
                            if pos >= 0:
                                after = line[pos + 3:]
                                if '*/' in after:
                                    after = after.replace('*/', '')
                                    content_lines.append(after.strip())
                                else:
                                    content_lines.append(after.strip())
                        elif idx == j:
                            before = line[:line.find('*/')]
                            before = re.sub(r'^\s*\*\s?', '', before)
                            content_lines.append(before.strip())
                        else:
                            line = re.sub(r'^\s*\*\s?', '', line)
                            content_lines.append(line.strip())
                    return '\n'.join([l for l in content_lines if l])
                k -= 1
            return None
        j -= 1
    return None


def find_matching_brace(lines: List[str], start_line: int, start_col: int) -> tuple:
    """Find the matching closing brace for a block starting at start_line, start_col."""
    depth = 0
    in_string = False
    in_char = False
    in_line_comment = False
    in_block_comment = False
    escape = False
    
    for line_idx in range(start_line, len(lines)):
        line = lines[line_idx]
        col_start = start_col if line_idx == start_line else 0
        for col in range(col_start, len(line)):
            ch = line[col]
            nxt = line[col + 1] if col + 1 < len(line) else ''
            
            if in_line_comment:
                break
            if in_block_comment:
                if ch == '*' and nxt == '/':
                    in_block_comment = False
                continue
            if in_string:
                if escape:
                    escape = False
                elif ch == '\\':
                    escape = True
                elif ch == '"':
                    in_string = False
                continue
            if in_char:
                if escape:
                    escape = False
                elif ch == '\\':
                    escape = True
                elif ch == "'":
                    in_char = False
                continue
            
            if ch == '/' and nxt == '/':
                in_line_comment = True
                continue
            if ch == '/' and nxt == '*':
                in_block_comment = True
                continue
            if ch == '"':
                in_string = True
                continue
            if ch == "'":
                # Could be char literal or lifetime - simple heuristic
                if col + 2 < len(line) and line[col + 2] == "'":
                    in_char = True
                continue
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    return line_idx, col + 1
    
    return len(lines) - 1, len(lines[-1]) if lines else 0


def parse_function(lines: List[str], start_idx: int, file_name: str) -> Optional[DocItem]:
    """Parse a function starting at the given line index (which contains 'pub fn' or 'pub async fn' or 'pub unsafe fn')."""
    # Gather the full function signature up to the opening brace or semicolon
    sig_lines = []
    i = start_idx
    depth_paren = 0
    depth_angle = 0
    in_where = False
    
    while i < len(lines):
        line = lines[i]
        sig_lines.append(line)
        
        # Count parens and angle brackets
        for ch in line:
            if ch == '(':
                depth_paren += 1
            elif ch == ')':
                depth_paren -= 1
            elif ch == '<':
                depth_angle += 1
            elif ch == '>':
                depth_angle -= 1
        
        stripped = line.strip()
        # Function body starts at { (when parens balanced) or ends with ;
        if depth_paren == 0:
            # Check for where clause
            if ' where' in line or line.rstrip().endswith('where'):
                in_where = True
            
            # Find { that's not in a string
            if '{' in line:
                # Verify the brace is real (not in string)
                # Simple heuristic: check if line doesn't end with just ';'
                # Actually, need to find the brace position carefully
                in_str = False
                esc = False
                for ci, ch in enumerate(line):
                    if esc:
                        esc = False
                        continue
                    if ch == '\\':
                        esc = True
                        continue
                    if ch == '"':
                        in_str = not in_str
                        continue
                    if not in_str and ch == '{':
                        # Found opening brace
                        if depth_angle <= 0 or 'where' in ' '.join(sig_lines):
                            # This is the body
                            pass
                        # Check if this is an impl block brace (shouldn't be for fn)
                        # For fn, the body brace is what we want
                        break
                else:
                    i += 1
                    continue
                # If line ends with '{' or contains '{' after balanced parens
                if depth_paren == 0 and depth_angle == 0:
                    if '{' in line:
                        break
            # Semicolon means function declaration (no body, e.g., in traits)
            if depth_paren == 0 and depth_angle == 0 and line.rstrip().endswith(';'):
                break
        
        i += 1
        if i >= len(lines):
            break
    
    # Extract signature text (everything before the { or ;)
    full_sig = ' '.join(sig_lines)
    # Remove trailing { ... } body
    brace_pos = -1
    in_str = False
    esc = False
    for ci, ch in enumerate(full_sig):
        if esc:
            esc = False
            continue
        if ch == '\\':
            esc = True
            continue
        if ch == '"':
            in_str = not in_str
            continue
        if not in_str and ch == '{':
            brace_pos = ci
            break
        if not in_str and ch == ';':
            brace_pos = ci
            break
    
    if brace_pos >= 0:
        sig = full_sig[:brace_pos].strip()
    else:
        sig = full_sig.strip()
    
    # Clean up whitespace
    sig = re.sub(r'\s+', ' ', sig)
    
    # Extract function name
    name_match = re.search(r'\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)', sig)
    if not name_match:
        return None
    name = name_match.group(1)
    
    # Clean up signature for display - make it multi-line if needed
    sig_display = sig.strip()
    
    # Extract comments
    comments = extract_doc_comments(lines, start_idx)
    if not comments:
        block = extract_block_doc_comment(lines, start_idx)
        if block:
            comments = block
    
    return DocItem(
        kind="function",
        name=name,
        signature=sig_display,
        comments=comments,
        file_name=file_name,
        line_number=start_idx + 1,
    )


def parse_struct(lines: List[str], start_idx: int, file_name: str) -> Optional[DocItem]:
    """Parse a struct starting at the given line index."""
    sig_lines = []
    i = start_idx
    brace_count = 0
    found_brace = False
    
    while i < len(lines):
        line = lines[i]
        sig_lines.append(line)
        
        stripped = line.strip()
        if '{' in line:
            found_brace = True
            # Find the matching closing brace
            start_col = line.index('{')
            end_line, end_col = find_matching_brace(lines, i, start_col)
            # Signature is up to the opening brace
            sig_text = ' '.join(sig_lines)
            brace_idx = sig_text.index('{')
            sig = sig_text[:brace_idx].strip()
            sig = re.sub(r'\s+', ' ', sig)
            
            name_match = re.search(r'\bstruct\s+([a-zA-Z_][a-zA-Z0-9_]*)', sig)
            if not name_match:
                return None
            name = name_match.group(1)
            
            comments = extract_doc_comments(lines, start_idx)
            if not comments:
                block = extract_block_doc_comment(lines, start_idx)
                if block:
                    comments = block
            
            return DocItem(
                kind="struct",
                name=name,
                signature=sig,
                comments=comments,
                file_name=file_name,
                line_number=start_idx + 1,
            )
        elif stripped.endswith(';'):
            # Tuple struct or unit struct
            sig_text = ' '.join(sig_lines)
            semi_idx = sig_text.rindex(';')
            sig = sig_text[:semi_idx].strip()
            sig = re.sub(r'\s+', ' ', sig)
            
            name_match = re.search(r'\bstruct\s+([a-zA-Z_][a-zA-Z0-9_]*)', sig)
            if not name_match:
                return None
            name = name_match.group(1)
            
            comments = extract_doc_comments(lines, start_idx)
            if not comments:
                block = extract_block_doc_comment(lines, start_idx)
                if block:
                    comments = block
            
            return DocItem(
                kind="struct",
                name=name,
                signature=sig,
                comments=comments,
                file_name=file_name,
                line_number=start_idx + 1,
            )
        i += 1
    
    return None


def is_pub_visibility(line: str) -> bool:
    """Check if a line has 'pub' visibility but not 'pub(crate)', 'pub(super)', etc.
    We include pub(crate) as well? The user said 'specifically public (pub keyword)'.
    We'll match 'pub ' but exclude pub(crate), pub(super), pub(in ...)."""
    stripped = line.strip()
    # Check for pub keyword
    match = re.match(r'^\s*pub\b', stripped)
    if not match:
        return False
    # Exclude pub(...) forms that aren't pub(crate)
    # Actually, user said ONLY pub keyword - let's include pub, pub(crate) too?
    # "specifically public (pub keyword)" - I'll interpret as plain `pub` without restrictions
    # But to be safe, let's include pub(crate) as it's still somewhat public
    # Actually re-reading: "specifically public (pub keyword)" - only plain pub
    after = stripped[3:].lstrip()
    if after.startswith('('):
        # pub(crate) or pub(super) or pub(in path) - exclude these
        return False
    return True


def should_exclude(file_path: str) -> bool:
    """Check if a file should be excluded from processing."""
    return os.path.basename(file_path) in EXCLUDED_FILES


def process_file(file_path: str) -> List[DocItem]:
    """Process a single Rust file and extract public items."""
    # Skip excluded files
    if should_exclude(file_path):
        return []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    lines = content.split('\n')
    file_name = os.path.basename(file_path)
    items = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Check for pub struct
        if re.match(r'^\s*pub\s+struct\s+', stripped) or re.match(r'^\s*pub\s+\(.*\)\s+struct\s+', stripped):
            # But only if it's exactly pub without restricted visibility
            if is_pub_visibility(stripped):
                item = parse_struct(lines, i, file_name)
                if item:
                    items.append(item)
        
        # Check for pub fn, pub async fn, pub unsafe fn, pub const fn, pub extern fn
        fn_patterns = [
            r'^\s*pub\s+fn\s+',
            r'^\s*pub\s+async\s+fn\s+',
            r'^\s*pub\s+unsafe\s+fn\s+',
            r'^\s*pub\s+const\s+fn\s+',
            r'^\s*pub\s+extern\s+.*\bfn\s+',
            r'^\s*pub\s+async\s+unsafe\s+fn\s+',
        ]
        
        for pattern in fn_patterns:
            if re.match(pattern, stripped):
                if is_pub_visibility(stripped):
                    item = parse_function(lines, i, file_name)
                    if item:
                        items.append(item)
                break
        
        i += 1
    
    return items


def process_directory(dir_path: str) -> dict:
    """Process all .rs files in a directory recursively."""
    results = {}
    for root, dirs, files in os.walk(dir_path):
        # Skip target directory
        if 'target' in dirs:
            dirs.remove('target')
        for f in sorted(files):
            if f.endswith('.rs'):
                if f in EXCLUDED_FILES:
                    continue
                full_path = os.path.join(root, f)
                rel_path = os.path.relpath(full_path, dir_path)
                items = process_file(full_path)
                if items:
                    results[rel_path] = items
    return results


def generate_markdown(items_by_file: dict, title: str = "Rust Public API Bindings") -> str:
    """Generate markdown documentation from collected items."""
    md = []
    md.append(f"# {title}\n")
    
    # Table of contents
    md.append("## Table of Contents\n")
    for file_name in sorted(items_by_file.keys()):
        items = items_by_file[file_name]
        md.append(f"- **{file_name}**")
        for item in items:
            anchor = f"{file_name.replace('.', '').replace('/', '').replace('_', '-')}-{item.name}"
            md.append(f"  - [{item.kind} `{item.name}`](#{anchor})")
    md.append("")
    
    md.append("---\n")
    
    for file_name in sorted(items_by_file.keys()):
        items = items_by_file[file_name]
        if not items:
            continue
        
        md.append(f"## File: `{file_name}`\n")
        
        for item in items:
            md.append(f"### {item.kind.upper()}: `{item.name}`\n")
            
            if item.comments:
                md.append("**Documentation:**\n")
                md.append(f"```")
                md.append(item.comments)
                md.append(f"```\n")
            else:
                md.append("**Documentation:** *No documentation comments found.*\n")
            
            md.append(f"**Signature:**\n")
            md.append(f"```rust")
            md.append(item.signature)
            md.append(f"```\n")
            
            md.append(f"**Location:** `{item.file_name}:{item.line_number}`\n")
            md.append("---\n")
    
    return '\n'.join(md)


def main():
    parser = argparse.ArgumentParser(
        description="Generate markdown bindings for public Rust functions and structs."
    )
    parser.add_argument(
        'input',
        help="Path to a .rs file or directory containing Rust source files"
    )
    parser.add_argument(
        '-o', '--output',
        default='bindings.md',
        help="Output markdown file (default: bindings.md)"
    )
    parser.add_argument(
        '--title',
        default='Rust Public API Bindings',
        help="Title for the markdown document"
    )
    
    args = parser.parse_args()
    
    input_path = Path(args.input)
    if not input_path.exists():
        print(f"Error: '{args.input}' does not exist.", file=sys.stderr)
        sys.exit(1)
    
    if input_path.is_file():
        if not input_path.suffix == '.rs':
            print(f"Error: '{args.input}' is not a .rs file.", file=sys.stderr)
            sys.exit(1)
        if should_exclude(str(input_path)):
            print(f"Error: '{args.input}' is in the exclusion list ({', '.join(EXCLUDED_FILES)}).", file=sys.stderr)
            sys.exit(1)
        items = process_file(str(input_path))
        items_by_file = {os.path.basename(str(input_path)): items} if items else {}
    else:
        items_by_file = process_directory(str(input_path))
    
    if not items_by_file:
        print("No public functions or structs found.", file=sys.stderr)
        sys.exit(0)
    
    markdown = generate_markdown(items_by_file, args.title)
    
    with open(args.output, 'w', encoding='utf-8') as f:
        f.write(markdown)
    
    total = sum(len(items) for items in items_by_file.values())
    print(f"Generated {args.output} with {total} public items across {len(items_by_file)} file(s).")
    print(f"Excluded files: {', '.join(sorted(EXCLUDED_FILES))}")


if __name__ == '__main__':
    main()
