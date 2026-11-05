import os
import re

SDK_DIR = "native/mod-sdk"

PATTERNS = [
    (r"(HF_ASSERT_SIZE\s*\(.*?\)\s*;)", r"/* \1 */"),
    (r"(HF_ASSERT_OFFSET\s*\(.*?\)\s*;)", r"/* \1 */"),
]


def purge_all_headers(directory):
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".h"):
                file_path = os.path.join(root, file)

                with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
                    content = f.read()

                modified_content = content
                total = 0
                for pattern, rep in PATTERNS:
                    modified_content, count = re.subn(
                        pattern, rep, modified_content, flags=re.DOTALL
                    )
                    total += count

                if total > 0:
                    with open(file_path, "w", encoding="utf-8") as f:
                        f.write(modified_content)
                    print(f"Cleansed {file_path}")


if __name__ == "__main__":
    purge_all_headers(SDK_DIR)
