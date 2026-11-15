// Co-op messages and the game's RPCs, from `native/mod-sdk/hellforge_net.h`
// and its generated `hellforge_rpcs.h`.
//
// `send` and the shared table are the mod channel (SDK 16); `rpc_listen` and
// `rpc_send` are the game's own network messages (SDK 17), whose names,
// argument structs, readers and senders `hellforge_rpcs.h` generates.

#![allow(dead_code)]

use crate::hf_api::{HF_API, HellForgeApi};
use crate::rust_ffi::{
    HF_ON_MESSAGE, HF_ON_SHARE, HF_RPC_DROP, HF_RPC_KEEP, HF_RPC_KIND_ARRAY, HF_RPC_KIND_BOOL,
    HF_RPC_KIND_DIRECTION, HF_RPC_KIND_FLOAT, HF_RPC_KIND_ID64, HF_RPC_KIND_INT,
    HF_RPC_KIND_QUATERNION, HF_RPC_KIND_STRING, HF_RPC_KIND_U32, HF_RPC_KIND_U64,
    HF_RPC_KIND_VECTOR2, HF_RPC_KIND_VECTOR3, HfEventInfo, HfMessageEvent, HfMod, HfRpcArg,
    HfRpcEvent, HfShareEvent,
};
use std::ffi::{CString, c_void};
use std::mem::size_of;
use std::os::raw::c_int;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::slice;
use std::sync::Mutex;

/// `HF_MESSAGE_MAX`: the largest mod message `send` carries.
pub const MESSAGE_MAX: usize = 1024;

/// `HF_SHARE_VALUE_MAX`: the largest value a shared-table row holds.
pub const SHARE_VALUE_MAX: usize = 64;

/// `HF_SHARE_ROWS`: the most rows one mod's shared table holds.
pub const SHARE_ROWS: usize = 512;

// bindgen reads the header's `ull` targets as signed 32-bit values, so the
// safe targets are written out here.

/// `HF_TO_ALL`: every other machine of the lobby.
pub const TO_ALL: u64 = u64::MAX;

/// `HF_TO_HOST`: the lobby's host.
pub const TO_HOST: u64 = u64::MAX - 1;

/// A message from the same mod on another machine of the lobby.
///
/// It arrives through [`HellForgeApi::on_message`]; `data` is valid during the
/// callback only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MessageEvent<'a> {
    /// The sender's peer id (the host's when it came from the host).
    pub from: u64,
    /// The message, at most [`MESSAGE_MAX`] bytes.
    pub data: &'a [u8],
}

/// One row of this mod's shared table, as it arrived, changed or went away.
///
/// It arrives through [`HellForgeApi::on_share`]; `value` is valid during the
/// callback only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShareEvent<'a> {
    /// The row's key.
    pub key: u64,
    /// The row's value, at most [`SHARE_VALUE_MAX`] bytes; empty when the row
    /// was removed.
    pub value: &'a [u8],
}

/// The most arguments one RPC event may hold, as a sanity bound on a malformed
/// message.
const RPC_ARGS_MAX: usize = 4096;

/// The most bytes an RPC string may hold (the header's strings are at most
/// 2047 bytes).
const RPC_STRING_MAX: usize = 2047;

/// What an RPC listener returns.
///
/// [`Keep`](Self::Keep) lets the message go on to later listeners and then to
/// the game's own receiver; [`Drop`](Self::Drop) stops it here, as if it never
/// arrived.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RpcAction {
    /// Let the message go on.
    Keep,
    /// Stop the message here.
    Drop,
}

impl RpcAction {
    const fn as_raw(self) -> c_int {
        match self {
            RpcAction::Keep => HF_RPC_KEEP as c_int,
            RpcAction::Drop => HF_RPC_DROP as c_int,
        }
    }
}

/// One argument of an RPC, as [`HellForgeApi::rpc_send`] passes it.
///
/// The variants name the game's `HF_RPC_KIND_*` types. An array is its element
/// arguments in order; its length is sent as the array's count.
#[derive(Clone, Debug, PartialEq)]
pub enum RpcArg<'a> {
    /// A `bool` (the game passes it as an int).
    Bool(bool),
    /// An `int`.
    Int(i32),
    /// A `float`.
    Float(f32),
    /// A `vector3`.
    Vector3([f32; 3]),
    /// A `quaternion` (`x`, `y`, `z`, `w`).
    Quaternion([f32; 4]),
    /// A NUL-terminated string borrowed for the call.
    String(&'a str),
    /// A `u32`.
    U32(u32),
    /// An `id64` (a name hash).
    Id64(u64),
    /// A `u64`.
    U64(u64),
    /// An array of arguments.
    Array(Vec<RpcArg<'a>>),
    /// A `direction`.
    Direction([f32; 3]),
    /// A `vector2`.
    Vector2([f32; 2]),
}

/// An [`RpcArg`] whose bytes and strings live through the loader's call.
enum OwnedArg {
    Bool(i32),
    Int(i32),
    Float(f32),
    Vector3([f32; 3]),
    Quaternion([f32; 4]),
    String(CString),
    U32(u32),
    Id64(u64),
    U64(u64),
    Array(Vec<OwnedArg>),
    Direction([f32; 3]),
    Vector2([f32; 2]),
}

impl OwnedArg {
    fn from_arg(arg: &RpcArg<'_>) -> Option<Self> {
        Some(match arg {
            RpcArg::Bool(value) => OwnedArg::Bool(i32::from(*value)),
            RpcArg::Int(value) => OwnedArg::Int(*value),
            RpcArg::Float(value) => OwnedArg::Float(*value),
            RpcArg::Vector3(value) => OwnedArg::Vector3(*value),
            RpcArg::Quaternion(value) => OwnedArg::Quaternion(*value),
            RpcArg::String(value) => OwnedArg::String(CString::new(*value).ok()?),
            RpcArg::U32(value) => OwnedArg::U32(*value),
            RpcArg::Id64(value) => OwnedArg::Id64(*value),
            RpcArg::U64(value) => OwnedArg::U64(*value),
            RpcArg::Array(values) => {
                OwnedArg::Array(values.iter().map(Self::from_arg).collect::<Option<_>>()?)
            }
            RpcArg::Direction(value) => OwnedArg::Direction(*value),
            RpcArg::Vector2(value) => OwnedArg::Vector2(*value),
        })
    }

    fn push_ffi(&self, out: &mut Vec<HfRpcArg>) {
        fn push(out: &mut Vec<HfRpcArg>, kind: u32, size: usize, value: *const c_void) {
            out.push(HfRpcArg {
                kind,
                size: size as u32,
                value,
            });
        }

        match self {
            OwnedArg::Bool(value) => push(
                out,
                HF_RPC_KIND_BOOL,
                size_of::<i32>(),
                value as *const i32 as *const c_void,
            ),
            OwnedArg::Int(value) => push(
                out,
                HF_RPC_KIND_INT,
                size_of::<i32>(),
                value as *const i32 as *const c_void,
            ),
            OwnedArg::Float(value) => push(
                out,
                HF_RPC_KIND_FLOAT,
                size_of::<f32>(),
                value as *const f32 as *const c_void,
            ),
            OwnedArg::Vector3(value) => push(
                out,
                HF_RPC_KIND_VECTOR3,
                size_of::<[f32; 3]>(),
                value as *const [f32; 3] as *const c_void,
            ),
            OwnedArg::Quaternion(value) => push(
                out,
                HF_RPC_KIND_QUATERNION,
                size_of::<[f32; 4]>(),
                value as *const [f32; 4] as *const c_void,
            ),
            OwnedArg::String(value) => push(
                out,
                HF_RPC_KIND_STRING,
                value.as_bytes().len(),
                value.as_ptr() as *const c_void,
            ),
            OwnedArg::U32(value) => push(
                out,
                HF_RPC_KIND_U32,
                size_of::<u32>(),
                value as *const u32 as *const c_void,
            ),
            OwnedArg::Id64(value) => push(
                out,
                HF_RPC_KIND_ID64,
                size_of::<u64>(),
                value as *const u64 as *const c_void,
            ),
            OwnedArg::U64(value) => push(
                out,
                HF_RPC_KIND_U64,
                size_of::<u64>(),
                value as *const u64 as *const c_void,
            ),
            OwnedArg::Array(values) => {
                push(out, HF_RPC_KIND_ARRAY, 0, values.len() as *const c_void);
                for value in values {
                    value.push_ffi(out);
                }
            }
            OwnedArg::Direction(value) => push(
                out,
                HF_RPC_KIND_DIRECTION,
                size_of::<[f32; 3]>(),
                value as *const [f32; 3] as *const c_void,
            ),
            OwnedArg::Vector2(value) => push(
                out,
                HF_RPC_KIND_VECTOR2,
                size_of::<[f32; 2]>(),
                value as *const [f32; 2] as *const c_void,
            ),
        }
    }
}

/// One RPC this machine received (`HfRpcEvent`).
///
/// It is valid during the listener's call only; [`rpc_listen`](HellForgeApi::rpc_listen)
/// passes a reference to it. Read its arguments with [`reader`](Self::reader),
/// or decode a named RPC with [`RpcMessage::decode`].
#[derive(Clone, Copy, Debug)]
pub struct RpcEvent<'a> {
    sender: u64,
    rpc: u32,
    args: &'a [HfRpcArg],
}

impl<'a> RpcEvent<'a> {
    /// The peer that sent it.
    pub const fn sender(&self) -> u64 {
        self.sender
    }

    /// Its id (`HF_RPC_...`, or [`rpc::ALL`] in `rpc_listen`).
    pub const fn rpc(&self) -> u32 {
        self.rpc
    }

    /// How many argument entries it has, including each array's count entry.
    pub const fn count(&self) -> usize {
        self.args.len()
    }

    /// The raw arguments, for callers that walk them themselves.
    pub const fn args(&self) -> &'a [HfRpcArg] {
        self.args
    }

    /// A cursor over its arguments.
    pub fn reader(&self) -> RpcReader<'a> {
        RpcReader {
            args: self.args,
            at: 0,
        }
    }

    unsafe fn from_raw<'b>(event: *const HfRpcEvent) -> RpcEvent<'b> {
        let event = unsafe { &*event };
        let count = event.count as usize;
        let args = if count == 0 || count > RPC_ARGS_MAX || event.args.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(event.args, count) }
        };

        RpcEvent {
            sender: event.sender,
            rpc: event.rpc,
            args,
        }
    }
}

/// Reads the arguments of an [`RpcEvent`] in their declared order.
///
/// Each method returns `None` when the next argument is missing, has another
/// kind, or its value does not fit. [`finish`](Self::finish) checks that no
/// argument is left over.
pub struct RpcReader<'a> {
    args: &'a [HfRpcArg],
    at: usize,
}

impl<'a> RpcReader<'a> {
    fn take(&mut self, kind: u32) -> Option<&'a HfRpcArg> {
        let arg = self.args.get(self.at)?;
        if arg.kind != kind {
            return None;
        }

        self.at += 1;
        Some(arg)
    }

    fn read<T: Copy>(&mut self, kind: u32) -> Option<T> {
        let arg = self.take(kind)?;
        if arg.value.is_null() || (arg.size as usize) < size_of::<T>() {
            return None;
        }

        Some(unsafe { arg.value.cast::<T>().read_unaligned() })
    }

    /// The next argument as a `bool`.
    pub fn bool(&mut self) -> Option<bool> {
        self.read::<i32>(HF_RPC_KIND_BOOL).map(|value| value != 0)
    }

    /// The next argument as an `int`.
    pub fn int(&mut self) -> Option<i32> {
        self.read(HF_RPC_KIND_INT)
    }

    /// The next argument as a `float`.
    pub fn float(&mut self) -> Option<f32> {
        self.read(HF_RPC_KIND_FLOAT)
    }

    /// The next argument as a `vector3`.
    pub fn vector3(&mut self) -> Option<[f32; 3]> {
        self.read(HF_RPC_KIND_VECTOR3)
    }

    /// The next argument as a `quaternion`.
    pub fn quaternion(&mut self) -> Option<[f32; 4]> {
        self.read(HF_RPC_KIND_QUATERNION)
    }

    /// The next argument as a string. The bytes are copied, so the result
    /// outlives the event.
    pub fn string(&mut self) -> Option<String> {
        let arg = self.take(HF_RPC_KIND_STRING)?;
        let size = arg.size as usize;
        if arg.value.is_null() || size > RPC_STRING_MAX {
            return None;
        }

        let bytes = unsafe { slice::from_raw_parts(arg.value as *const u8, size) };
        Some(String::from_utf8_lossy(bytes).into_owned())
    }

    /// The next argument as a `u32`.
    pub fn u32(&mut self) -> Option<u32> {
        self.read(HF_RPC_KIND_U32)
    }

    /// The next argument as an `id64` (a name hash).
    pub fn id64(&mut self) -> Option<u64> {
        self.read(HF_RPC_KIND_ID64)
    }

    /// The next argument as a `u64`.
    pub fn u64(&mut self) -> Option<u64> {
        self.read(HF_RPC_KIND_U64)
    }

    /// The next argument as a `direction`.
    pub fn direction(&mut self) -> Option<[f32; 3]> {
        self.read(HF_RPC_KIND_DIRECTION)
    }

    /// The next argument as a `vector2`.
    pub fn vector2(&mut self) -> Option<[f32; 2]> {
        self.read(HF_RPC_KIND_VECTOR2)
    }

    /// The next argument as an array of at most `max` values, read by `read`.
    pub fn array<T>(
        &mut self,
        max: usize,
        mut read: impl FnMut(&mut Self) -> Option<T>,
    ) -> Option<Vec<T>> {
        let arg = self.take(HF_RPC_KIND_ARRAY)?;
        let count = arg.value as usize;
        if count > max {
            return None;
        }

        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(read(self)?);
        }

        Some(values)
    }

    /// Whether every argument was read.
    pub fn finish(&self) -> bool {
        self.at == self.args.len()
    }
}

/// The arguments of one of the game's RPCs.
///
/// Each RPC of `hellforge_rpcs.h` has a type in [`rpc`] implementing this:
/// [`HellForgeApi::rpc_send_message`] sends one and
/// [`HellForgeApi::rpc_listen_message`] receives one.
pub trait RpcMessage: Sized {
    /// The RPC's id (`HF_RPC_...`).
    const ID: u32;

    /// Decodes `event` as this RPC. Returns `None` when it is another RPC or
    /// its arguments do not match.
    fn decode(event: &RpcEvent<'_>) -> Option<Self>;

    /// Encodes this message as [`HellForgeApi::rpc_send`] arguments. Returns
    /// `None` when an array is longer than its RPC allows or a string has an
    /// interior NUL byte.
    fn encode(&self) -> Option<Vec<RpcArg<'_>>>;
}

unsafe extern "C" fn message_trampoline<F>(
    _mod: *mut HfMod,
    event: *const HfEventInfo,
    user: *mut c_void,
) where
    F: FnMut(&HellForgeApi, MessageEvent<'_>) + Send + 'static,
{
    if event.is_null() || user.is_null() {
        return;
    }

    let Some(api) = HF_API.get() else {
        return;
    };

    // The loader keeps this allocation for the session: on_message has no
    // unregister call, so it is not reclaimed here. A poisoned lock means a
    // previous invocation panicked; the closure is still valid, so run it
    // anyway.
    let callback = unsafe { &*(user as *const Mutex<F>) };
    let mut callback = match callback.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    let event = unsafe { &*(event as *const HfMessageEvent) };
    let bytes = (event.bytes as usize).min(MESSAGE_MAX);
    let event = MessageEvent {
        from: event.from,
        data: unsafe { slice::from_raw_parts(event.data.as_ptr(), bytes) },
    };

    if catch_unwind(AssertUnwindSafe(|| callback(api, event))).is_err() {
        api.log("rustforge: message callback panicked");
    }
}

unsafe extern "C" fn share_trampoline<F>(
    _mod: *mut HfMod,
    event: *const HfEventInfo,
    user: *mut c_void,
) where
    F: FnMut(&HellForgeApi, ShareEvent<'_>) + Send + 'static,
{
    if event.is_null() || user.is_null() {
        return;
    }

    let Some(api) = HF_API.get() else {
        return;
    };

    // As in message_trampoline: the loader owns the allocation for the session.
    let callback = unsafe { &*(user as *const Mutex<F>) };
    let mut callback = match callback.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    let event = unsafe { &*(event as *const HfShareEvent) };
    let bytes = (event.bytes as usize).min(SHARE_VALUE_MAX);
    let event = ShareEvent {
        key: event.key,
        value: unsafe { slice::from_raw_parts(event.value.as_ptr(), bytes) },
    };

    if catch_unwind(AssertUnwindSafe(|| callback(api, event))).is_err() {
        api.log("rustforge: share callback panicked");
    }
}

unsafe extern "C" fn rpc_trampoline<F>(
    _mod: *mut HfMod,
    event: *const HfRpcEvent,
    user: *mut c_void,
) -> c_int
where
    F: FnMut(&HellForgeApi, &RpcEvent<'_>) -> RpcAction + Send + 'static,
{
    if event.is_null() || user.is_null() {
        return RpcAction::Keep.as_raw();
    }

    let Some(api) = HF_API.get() else {
        return RpcAction::Keep.as_raw();
    };

    // As in message_trampoline: the loader owns the allocation for the session.
    let callback = unsafe { &*(user as *const Mutex<F>) };
    let mut callback = match callback.lock() {
        Ok(callback) => callback,
        Err(poisoned) => poisoned.into_inner(),
    };

    let event = unsafe { RpcEvent::from_raw(event) };
    match catch_unwind(AssertUnwindSafe(|| callback(api, &event))) {
        Ok(action) => action.as_raw(),
        Err(_) => {
            api.log("rustforge: RPC listener panicked");
            RpcAction::Keep.as_raw()
        }
    }
}

impl HellForgeApi {
    /// Sends a message to the same mod on other machines of the lobby.
    ///
    /// `to` is [`TO_ALL`], [`TO_HOST`], or a peer id from a
    /// [`MessageEvent::from`]. At most [`MESSAGE_MAX`] bytes. Returns `true`
    /// when the message was queued.
    pub fn send(&self, to: u64, data: &[u8]) -> bool {
        if !self.supports(16) || data.len() > MESSAGE_MAX {
            return false;
        }

        let Some(send) = self.raw_api().send else {
            return false;
        };
        let Ok(bytes) = u32::try_from(data.len()) else {
            return false;
        };

        unsafe { send(self.mod_handle(), to, data.as_ptr() as *const c_void, bytes) == 0 }
    }

    /// Sets a row of this mod's shared table (host only).
    ///
    /// An empty `value` removes the row. Returns `true` when set.
    pub fn share_set(&self, key: u64, value: &[u8]) -> bool {
        if !self.supports(16) || value.len() > SHARE_VALUE_MAX {
            return false;
        }

        let Some(share_set) = self.raw_api().share_set else {
            return false;
        };
        let Ok(bytes) = u32::try_from(value.len()) else {
            return false;
        };

        unsafe {
            share_set(
                self.mod_handle(),
                key,
                value.as_ptr() as *const c_void,
                bytes,
            ) == 0
        }
    }

    /// One row of this mod's shared table on this machine.
    ///
    /// Returns `None` when the row is absent.
    pub fn share_get(&self, key: u64) -> Option<Vec<u8>> {
        if !self.supports(16) {
            return None;
        }

        let share_get = self.raw_api().share_get?;
        let mut value = [0u8; SHARE_VALUE_MAX];
        let size = unsafe {
            share_get(
                self.mod_handle(),
                key,
                value.as_mut_ptr() as *mut c_void,
                SHARE_VALUE_MAX as u32,
            )
        } as usize;

        (size != 0).then(|| value[..size.min(SHARE_VALUE_MAX)].to_vec())
    }

    /// The `index`-th row of this mod's shared table on this machine.
    ///
    /// Returns the key and value, or `None` past the last row.
    pub fn share_row(&self, index: u32) -> Option<(u64, Vec<u8>)> {
        if !self.supports(16) {
            return None;
        }

        let share_row = self.raw_api().share_row?;
        let mut key = 0u64;
        let mut value = [0u8; SHARE_VALUE_MAX];
        let mut bytes = 0u32;
        let status = unsafe {
            share_row(
                self.mod_handle(),
                index,
                &mut key,
                value.as_mut_ptr() as *mut c_void,
                SHARE_VALUE_MAX as u32,
                &mut bytes,
            )
        };

        (status != 0).then(|| {
            let bytes = (bytes as usize).min(SHARE_VALUE_MAX);
            (key, value[..bytes].to_vec())
        })
    }

    /// Calls `callback` when the same mod on another machine sends a message
    /// (`HF_ON_MESSAGE`).
    ///
    /// The callback runs on the game thread, so it must be `Send`. It stays
    /// registered for the session (the API has no unregister call).
    /// Returns `true` when the listener was registered.
    pub fn on_message<F>(&self, callback: F) -> bool
    where
        F: FnMut(&HellForgeApi, MessageEvent<'_>) + Send + 'static,
    {
        if !self.supports(16) {
            return false;
        }

        let user = Box::into_raw(Box::new(Mutex::new(callback))) as *mut c_void;
        let registered =
            unsafe { self.subscribe(HF_ON_MESSAGE, Some(message_trampoline::<F>), user, 0) };

        if !registered {
            // The loader will never call the trampoline: take the allocation
            // back instead of leaking it.
            drop(unsafe { Box::from_raw(user as *mut Mutex<F>) });
            return false;
        }

        true
    }

    /// Calls `callback` when a row of this mod's shared table arrives, changes
    /// or goes (`HF_ON_SHARE`).
    ///
    /// The callback runs on the game thread, so it must be `Send`. It stays
    /// registered for the session (the API has no unregister call).
    /// Returns `true` when the listener was registered.
    pub fn on_share<F>(&self, callback: F) -> bool
    where
        F: FnMut(&HellForgeApi, ShareEvent<'_>) + Send + 'static,
    {
        if !self.supports(16) {
            return false;
        }

        let user = Box::into_raw(Box::new(Mutex::new(callback))) as *mut c_void;
        let registered =
            unsafe { self.subscribe(HF_ON_SHARE, Some(share_trampoline::<F>), user, 0) };

        if !registered {
            drop(unsafe { Box::from_raw(user as *mut Mutex<F>) });
            return false;
        }

        true
    }

    /// Listens to one of the game's RPCs as this machine receives it.
    ///
    /// `rpc` is its `HF_RPC_...` id (see [`rpc`]), or [`rpc::ALL`] for every
    /// one. Listeners run on the game thread before the game's own receiver,
    /// highest priority first (equal priorities in the order they listened).
    /// Call it from `#[hf_entry]`.
    ///
    /// Returns `true` when listening; `false` when refused (an RPC this build
    /// does not have, or a loader older than SDK 17).
    pub fn rpc_listen<F>(&self, rpc: u32, priority: i32, callback: F) -> bool
    where
        F: FnMut(&HellForgeApi, &RpcEvent<'_>) -> RpcAction + Send + 'static,
    {
        if !self.supports(17) {
            return false;
        }

        let Some(rpc_listen) = self.raw_api().rpc_listen else {
            return false;
        };

        let user = Box::into_raw(Box::new(Mutex::new(callback))) as *mut c_void;
        let status = unsafe {
            rpc_listen(
                self.mod_handle(),
                rpc,
                Some(rpc_trampoline::<F>),
                user,
                priority,
            )
        };

        if status != 0 {
            // The loader will never call the trampoline: take the allocation
            // back instead of leaking it.
            drop(unsafe { Box::from_raw(user as *mut Mutex<F>) });
            return false;
        }

        true
    }

    /// Sends one of the game's RPCs.
    ///
    /// `to` is [`TO_ALL`], [`TO_HOST`], or a peer from an
    /// [`RpcEvent::sender`]. The game encodes each argument by its declared
    /// type, so values outside a type's range are clamped or wrap. Call it on
    /// the game thread. Returns `true` when sent.
    pub fn rpc_send(&self, rpc: u32, to: u64, args: &[RpcArg<'_>]) -> bool {
        if !self.supports(17) {
            return false;
        }

        let Some(rpc_send) = self.raw_api().rpc_send else {
            return false;
        };

        let mut owned = Vec::with_capacity(args.len());
        for arg in args {
            let Some(arg) = OwnedArg::from_arg(arg) else {
                return false;
            };
            owned.push(arg);
        }

        let mut ffi = Vec::new();
        for arg in &owned {
            arg.push_ffi(&mut ffi);
        }

        let Ok(count) = u32::try_from(ffi.len()) else {
            return false;
        };
        let args = if ffi.is_empty() {
            std::ptr::null()
        } else {
            ffi.as_ptr()
        };

        unsafe { rpc_send(self.mod_handle(), rpc, to, args, count) == 0 }
    }

    /// Sends one of the game's RPCs from its typed message
    /// ([`RpcMessage`]).
    ///
    /// Returns `true` when sent; `false` when refused or when the message has
    /// an array longer than its RPC allows.
    pub fn rpc_send_message<M: RpcMessage>(&self, to: u64, message: &M) -> bool {
        match message.encode() {
            Some(args) => self.rpc_send(M::ID, to, &args),
            None => false,
        }
    }

    /// Listens to one of the game's RPCs and decodes it into its typed message
    /// ([`RpcMessage`]).
    ///
    /// Like [`rpc_listen`](Self::rpc_listen), but the callback receives the
    /// decoded message instead of the raw event. A message whose arguments do
    /// not decode is logged and left for the game's own receiver.
    ///
    /// Returns `true` when listening; `false` when refused.
    pub fn rpc_listen_message<M, F>(&self, priority: i32, callback: F) -> bool
    where
        M: RpcMessage + 'static,
        F: FnMut(&HellForgeApi, M) + Send + 'static,
    {
        let mut callback = callback;
        self.rpc_listen(M::ID, priority, move |api, event| {
            match M::decode(event) {
                Some(message) => {
                    callback(api, message);
                }
                None => api.log("rustforge: RPC arguments did not decode"),
            }

            RpcAction::Keep
        })
    }
}

// Each `rpc!` invocation declares one RPC of `hellforge_rpcs.h`: its id, its
// typed argument struct, and the reader and writer for `rpc_listen` and
// `rpc_send`.

macro_rules! rpc {
    (
        $(#[$meta:meta])*
        $name:ident / $id:ident / $value:literal {
            $($field:ident : $kind:tt),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            $(pub $field: rpc_field!(@type $kind),)*
        }

        #[doc = concat!("`HF_RPC_", stringify!($id), "`.")]
        pub const $id: u32 = $value;

        impl $crate::hf_net::RpcMessage for $name {
            const ID: u32 = $value;

            fn decode(event: &$crate::hf_net::RpcEvent<'_>) -> Option<Self> {
                if event.rpc() != Self::ID {
                    return None;
                }

                let mut reader = event.reader();
                let message = Self {
                    $($field: rpc_field!(@decode reader, $kind)?,)*
                };

                reader.finish().then_some(message)
            }

            #[allow(clippy::vec_init_then_push)]
            fn encode(&self) -> Option<Vec<$crate::hf_net::RpcArg<'_>>> {
                let mut args = Vec::new();
                $(rpc_field!(@encode args, &self.$field, $kind);)*
                Some(args)
            }
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident / $id:ident / $value:literal ;
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name;

        #[doc = concat!("`HF_RPC_", stringify!($id), "`.")]
        pub const $id: u32 = $value;

        impl $crate::hf_net::RpcMessage for $name {
            const ID: u32 = $value;

            fn decode(event: &$crate::hf_net::RpcEvent<'_>) -> Option<Self> {
                (event.rpc() == Self::ID && event.count() == 0).then_some(Self)
            }

            fn encode(&self) -> Option<Vec<$crate::hf_net::RpcArg<'_>>> {
                Some(Vec::new())
            }
        }
    };
}

macro_rules! rpc_field {
    (@type bool) => { bool };
    (@type int) => { i32 };
    (@type float) => { f32 };
    (@type vector3) => { [f32; 3] };
    (@type quaternion) => { [f32; 4] };
    (@type string) => { String };
    (@type u32) => { u32 };
    (@type id64) => { u64 };
    (@type u64) => { u64 };
    (@type direction) => { [f32; 3] };
    (@type vector2) => { [f32; 2] };
    (@type [$kind:ident; $max:literal]) => { Vec<rpc_field!(@type $kind)> };

    (@decode $reader:expr, bool) => { $reader.bool() };
    (@decode $reader:expr, int) => { $reader.int() };
    (@decode $reader:expr, float) => { $reader.float() };
    (@decode $reader:expr, vector3) => { $reader.vector3() };
    (@decode $reader:expr, quaternion) => { $reader.quaternion() };
    (@decode $reader:expr, string) => { $reader.string() };
    (@decode $reader:expr, u32) => { $reader.u32() };
    (@decode $reader:expr, id64) => { $reader.id64() };
    (@decode $reader:expr, u64) => { $reader.u64() };
    (@decode $reader:expr, direction) => { $reader.direction() };
    (@decode $reader:expr, vector2) => { $reader.vector2() };
    (@decode $reader:expr, [$kind:ident; $max:literal]) => {
        $reader.array($max, |reader| rpc_field!(@decode reader, $kind))
    };

    (@encode $args:expr, $value:expr, bool) => {
        $args.push($crate::hf_net::RpcArg::Bool(*$value));
    };
    (@encode $args:expr, $value:expr, int) => {
        $args.push($crate::hf_net::RpcArg::Int(*$value));
    };
    (@encode $args:expr, $value:expr, float) => {
        $args.push($crate::hf_net::RpcArg::Float(*$value));
    };
    (@encode $args:expr, $value:expr, vector3) => {
        $args.push($crate::hf_net::RpcArg::Vector3(*$value));
    };
    (@encode $args:expr, $value:expr, quaternion) => {
        $args.push($crate::hf_net::RpcArg::Quaternion(*$value));
    };
    (@encode $args:expr, $value:expr, string) => {
        $args.push($crate::hf_net::RpcArg::String(($value).as_str()));
    };
    (@encode $args:expr, $value:expr, u32) => {
        $args.push($crate::hf_net::RpcArg::U32(*$value));
    };
    (@encode $args:expr, $value:expr, id64) => {
        $args.push($crate::hf_net::RpcArg::Id64(*$value));
    };
    (@encode $args:expr, $value:expr, u64) => {
        $args.push($crate::hf_net::RpcArg::U64(*$value));
    };
    (@encode $args:expr, $value:expr, direction) => {
        $args.push($crate::hf_net::RpcArg::Direction(*$value));
    };
    (@encode $args:expr, $value:expr, vector2) => {
        $args.push($crate::hf_net::RpcArg::Vector2(*$value));
    };
    (@encode $args:expr, $value:expr, [$kind:ident; $max:literal]) => {{
        if $value.len() > $max {
            return None;
        }

        let values = $value
            .iter()
            .map(|value| rpc_field!(@arg value, $kind))
            .collect();
        $args.push($crate::hf_net::RpcArg::Array(values));
    }};

    (@arg $value:expr, bool) => { $crate::hf_net::RpcArg::Bool(*$value) };
    (@arg $value:expr, int) => { $crate::hf_net::RpcArg::Int(*$value) };
    (@arg $value:expr, float) => { $crate::hf_net::RpcArg::Float(*$value) };
    (@arg $value:expr, vector3) => { $crate::hf_net::RpcArg::Vector3(*$value) };
    (@arg $value:expr, quaternion) => { $crate::hf_net::RpcArg::Quaternion(*$value) };
    (@arg $value:expr, string) => { $crate::hf_net::RpcArg::String(($value).as_str()) };
    (@arg $value:expr, u32) => { $crate::hf_net::RpcArg::U32(*$value) };
    (@arg $value:expr, id64) => { $crate::hf_net::RpcArg::Id64(*$value) };
    (@arg $value:expr, u64) => { $crate::hf_net::RpcArg::U64(*$value) };
    (@arg $value:expr, direction) => { $crate::hf_net::RpcArg::Direction(*$value) };
    (@arg $value:expr, vector2) => { $crate::hf_net::RpcArg::Vector2(*$value) };
}

/// The game's RPCs by name, from `hellforge_rpcs.h`.
///
/// Each type here is one RPC's arguments; it implements [`RpcMessage`] so
/// [`HellForgeApi::rpc_send_message`] can send it and
/// [`HellForgeApi::rpc_listen_message`] can receive it. The `RPC_*` names are
/// the RPCs' ids for [`HellForgeApi::rpc_send`] and [`HellForgeApi::rpc_listen`].
pub mod rpc {
    /// `HF_RPC_ALL`: every RPC, for a listener that logs or filters all
    /// messages.
    pub const ALL: u32 = crate::rust_ffi::HF_RPC_ALL;

    rpc! {
        /// outpost_location_discovered (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        OutpostLocationDiscovered / OUTPOST_LOCATION_DISCOVERED / 0x00019F67 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// set_light_intensity (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `arg2`: float, 8 bits over 0..1
        SetLightIntensity / SET_LIGHT_INTENSITY / 0x00CBA0F0 {
            entity: int,
            id_string32: u32,
            arg2: float,
        }
    }

    rpc! {
        /// sync_remove_lobby:
        ///
        /// Arguments:
        /// - `int_value`: int, 32 bits, 0..0
        /// - `network_string`: string, at most 2047 bytes
        SyncRemoveLobby / SYNC_REMOVE_LOBBY / 0x02337267 {
            int_value: int,
            network_string: string,
        }
    }

    rpc! {
        /// remove_all_ai (session-bound): no arguments.
        RemoveAllAi / REMOVE_ALL_AI / 0x02F892C7;
    }

    rpc! {
        /// stop_looping_surface_effect (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 2 bits, 0..3
        StopLoopingSurfaceEffect / STOP_LOOPING_SURFACE_EFFECT / 0x037BDF09 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// set_visible (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetVisible / SET_VISIBLE / 0x0411D65B {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// set_interactable_zone_occupant (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `peer_id`: u64
        SetInteractableZoneOccupant / SET_INTERACTABLE_ZONE_OCCUPANT / 0x0434CD28 {
            entity: int,
            unsigned_value: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// pickup_entity (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        PickupEntity / PICKUP_ENTITY / 0x04365DCF {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// force_exit_seat_collection_retry (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `seat_index`: int, 5 bits, -1..30
        ForceExitSeatCollectionRetry / FORCE_EXIT_SEAT_COLLECTION_RETRY / 0x04506CD6 {
            entity: int,
            seat_index: int,
        }
    }

    rpc! {
        /// set_entrance_opened (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        /// - `bool_value`: bool
        SetEntranceOpened / SET_ENTRANCE_OPENED / 0x048CE9F5 {
            entity: int,
            arg1: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// start_throwing (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 48 x u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        StartThrowing / START_THROWING / 0x04D75D0B {
            entity: int,
            arg1: [u64; 48],
            unsigned_value: int,
        }
    }

    rpc! {
        /// game_over (session-bound):
        ///
        /// Arguments:
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        GameOver / GAME_OVER / 0x063A7A75 {
            bool_value: bool,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// from_host_synchronized_tasks_done (session-bound): no arguments.
        FromHostSynchronizedTasksDone / FROM_HOST_SYNCHRONIZED_TASKS_DONE / 0x06F30F4A;
    }

    rpc! {
        /// on_equipment_changed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        OnEquipmentChanged / ON_EQUIPMENT_CHANGED / 0x07319332 {
            entity: int,
        }
    }

    rpc! {
        /// set_living_effect (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        SetLivingEffect / SET_LIVING_EFFECT / 0x07384879 {
            entity: int,
            unsigned_value: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// vehicle_crash (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        /// - `vector3`: vector3, 31 bits over -10000..10000
        VehicleCrash / VEHICLE_CRASH / 0x0770D3CD {
            entity: int,
            entity_2: int,
            float_value: float,
            bool_value: bool,
            vector3: vector3,
        }
    }

    rpc! {
        /// set_player_cursor_active (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        SetPlayerCursorActive / SET_PLAYER_CURSOR_ACTIVE / 0x0883C6E3 {
            unsigned_value: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// from_partyleader_join_lobby:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: up to 5 x u64
        /// - `network_string`: string, at most 2047 bytes
        /// - `peer_id`: u64
        FromPartyleaderJoinLobby / FROM_PARTYLEADER_JOIN_LOBBY / 0x0887CDAB {
            arg0: u64,
            arg1: [u64; 5],
            network_string: string,
            peer_id: u64,
        }
    }

    rpc! {
        /// from_host_peer_disconnected (session-bound): no arguments.
        FromHostPeerDisconnected / FROM_HOST_PEER_DISCONNECTED / 0x08E90BA8;
    }

    rpc! {
        /// clear_tag (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `tag`: u64
        ClearTag / CLEAR_TAG / 0x095ED3B5 {
            entity: int,
            tag: u64,
        }
    }

    rpc! {
        /// restock (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        Restock / RESTOCK / 0x0982737B {
            entity: int,
        }
    }

    rpc! {
        /// audio_event_no_soundscape (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        AudioEventNoSoundscape / AUDIO_EVENT_NO_SOUNDSCAPE / 0x09BA1EA5 {
            entity: int,
            id_string32: u32,
            id_string32_2: u32,
        }
    }

    rpc! {
        /// interrupt_ability (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        InterruptAbility / INTERRUPT_ABILITY / 0x0A882BBD {
            entity: int,
        }
    }

    rpc! {
        /// set_minigame_switches_selected_index (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetMinigameSwitchesSelectedIndex / SET_MINIGAME_SWITCHES_SELECTED_INDEX / 0x0A99DE93 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// hotjoin_sync_objective_link_states (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: up to 64 x int, 32 bits, 0..0
        HotjoinSyncObjectiveLinkStates / HOTJOIN_SYNC_OBJECTIVE_LINK_STATES / 0x0B7AF0A3 {
            entity: int,
            unsigned_value: int,
            arg2: [int; 64],
        }
    }

    rpc! {
        /// request_skip_dialogue (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        RequestSkipDialogue / REQUEST_SKIP_DIALOGUE / 0x0D4153F5 {
            entity: int,
        }
    }

    rpc! {
        /// from_host_client_ready (session-bound): no arguments.
        FromHostClientReady / FROM_HOST_CLIENT_READY / 0x0D4FD257;
    }

    rpc! {
        /// set_zone_armor (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetZoneArmor / SET_ZONE_ARMOR / 0x0DCD2978 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// drop_payload (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        DropPayload / DROP_PAYLOAD / 0x0DDC9FEF {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// damage_zones_state_changed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 2 x u64
        /// - `peer_id`: u64
        DamageZonesStateChanged / DAMAGE_ZONES_STATE_CHANGED / 0x0E53F7A4 {
            entity: int,
            arg1: [u64; 2],
            peer_id: u64,
        }
    }

    rpc! {
        /// on_gore_damage (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 5 bits, 0..31
        /// - `damage_type`: int, 4 bits, 0..15
        /// - `element_type`: int, 4 bits, 0..15
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `id_string32`: u32
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        OnGoreDamage / ON_GORE_DAMAGE / 0x0E57D84B {
            entity: int,
            arg1: int,
            damage_type: int,
            element_type: int,
            unsigned_value: int,
            unsigned_value_2: int,
            vector3: vector3,
            id_string32: u32,
            bool_value: bool,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// deform_terrain (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        DeformTerrain / DEFORM_TERRAIN / 0x0FC01D57 {
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// shared_stratagem_state:
        ///
        /// Arguments:
        /// - `arg0`: int, 8 bits, 0..255
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: int, 2 bits, 0..3
        SharedStratagemState / SHARED_STRATAGEM_STATE / 0x0FC123A3 {
            arg0: int,
            unsigned_value: int,
            arg2: int,
        }
    }

    rpc! {
        /// fire_mounted_weapon (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        FireMountedWeapon / FIRE_MOUNTED_WEAPON / 0x0FE9FD4B {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// animation_event_speed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `animation_event`: int, 13 bits, 0..8191
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        AnimationEventSpeed / ANIMATION_EVENT_SPEED / 0x10722849 {
            entity: int,
            animation_event: int,
            float_value: float,
            bool_value: bool,
        }
    }

    rpc! {
        /// set_minigame_keypad_selected_index (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetMinigameKeypadSelectedIndex / SET_MINIGAME_KEYPAD_SELECTED_INDEX / 0x10CB40C6 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// set_ragdoll_penetration_resolution_vector (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `bool_value`: bool
        SetRagdollPenetrationResolutionVector / SET_RAGDOLL_PENETRATION_RESOLUTION_VECTOR / 0x112A8A3B {
            entity: int,
            vector3: vector3,
            bool_value: bool,
        }
    }

    rpc! {
        /// request_host_sync_info (session-bound): no arguments.
        RequestHostSyncInfo / REQUEST_HOST_SYNC_INFO / 0x11645E27;
    }

    rpc! {
        /// record_discovery_interact (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        RecordDiscoveryInteract / RECORD_DISCOVERY_INTERACT / 0x1204D560 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// on_destroyed (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `arg4`: int, 8 bits, 0..255
        /// - `arg5`: int, 3 bits, 0..7
        /// - `arg6`: int, 1 bits, 0..1
        /// - `peer_id`: u64
        OnDestroyed / ON_DESTROYED / 0x120872FE {
            unsigned_value: int,
            vector3: vector3,
            vector3_2: vector3,
            unsigned_value_2: int,
            arg4: int,
            arg5: int,
            arg6: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// wound_event_list (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 32 x u64
        WoundEventList / WOUND_EVENT_LIST / 0x125042C9 {
            entity: int,
            arg1: [u64; 32],
        }
    }

    rpc! {
        /// start_follow_curve (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `arg4`: up to 64 x vector3, 31 bits over -10000..10000
        /// - `arg5`: up to 64 x float, 31 bits over -10000..10000
        StartFollowCurve / START_FOLLOW_CURVE / 0x12FF1D89 {
            entity: int,
            unsigned_value: int,
            float_value: float,
            unsigned_value_2: int,
            arg4: [vector3; 64],
            arg5: [float; 64],
        }
    }

    rpc! {
        /// from_migrated_host_peer_state:
        ///
        /// Arguments:
        /// - `peer_state`: int, 4 bits, 0..15
        FromMigratedHostPeerState / FROM_MIGRATED_HOST_PEER_STATE / 0x1346ED6F {
            peer_state: int,
        }
    }

    rpc! {
        /// from_host_level_generation_pass_completed (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg1`: u64
        FromHostLevelGenerationPassCompleted / FROM_HOST_LEVEL_GENERATION_PASS_COMPLETED / 0x134D1D7E {
            unsigned_value: int,
            arg1: u64,
        }
    }

    rpc! {
        /// stop_particle_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 5 bits, 0..31
        StopParticleEvent / STOP_PARTICLE_EVENT / 0x136B7300 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// entity_deletion_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        EntityDeletionRequest / ENTITY_DELETION_REQUEST / 0x14B715B6 {
            entity: int,
        }
    }

    rpc! {
        /// reset_uoi_interact (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        ResetUoiInteract / RESET_UOI_INTERACT / 0x1682B091 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// request_ingame_ship_sync (session-bound):
        ///
        /// Arguments:
        /// - `peer_id`: u64
        RequestIngameShipSync / REQUEST_INGAME_SHIP_SYNC / 0x169C89F0 {
            peer_id: u64,
        }
    }

    rpc! {
        /// join_voice_room:
        ///
        /// Arguments:
        /// - `arg0`: u64
        JoinVoiceRoom / JOIN_VOICE_ROOM / 0x176A7787 {
            arg0: u64,
        }
    }

    rpc! {
        /// reveal_map (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        RevealMap / REVEAL_MAP / 0x17E49104 {
            entity: int,
        }
    }

    rpc! {
        /// sync_party_clan (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `network_string`: string, at most 2047 bytes
        /// - `network_string_2`: string, at most 2047 bytes
        SyncPartyClan / SYNC_PARTY_CLAN / 0x1860C6E4 {
            unsigned_value: int,
            network_string: string,
            network_string_2: string,
        }
    }

    rpc! {
        /// reset_shield_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ResetShieldState / RESET_SHIELD_STATE / 0x18DF1D85 {
            entity: int,
        }
    }

    rpc! {
        /// sync_galactic_war_planet_campaign_info:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `arg3`: int, 3 bits, 0..7
        /// - `arg4`: int, 3 bits, 0..7
        SyncGalacticWarPlanetCampaignInfo / SYNC_GALACTIC_WAR_PLANET_CAMPAIGN_INFO / 0x19339CD6 {
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
            arg3: int,
            arg4: int,
        }
    }

    rpc! {
        /// set_minigame_grid_maze_position (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        SetMinigameGridMazePosition / SET_MINIGAME_GRID_MAZE_POSITION / 0x199CAEE1 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
        }
    }

    rpc! {
        /// from_host_latest_config_already_synced: no arguments.
        FromHostLatestConfigAlreadySynced / FROM_HOST_LATEST_CONFIG_ALREADY_SYNCED / 0x19FC9ABC;
    }

    rpc! {
        /// from_host_reset_confirmed_peers: no arguments.
        FromHostResetConfirmedPeers / FROM_HOST_RESET_CONFIRMED_PEERS / 0x1AB72826;
    }

    rpc! {
        /// from_host_peer_removed:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: int, 4 bits, 0..15
        FromHostPeerRemoved / FROM_HOST_PEER_REMOVED / 0x1ACFCDF1 {
            peer_id: u64,
            arg1: int,
        }
    }

    rpc! {
        /// change_corpse_collision (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ChangeCorpseCollision / CHANGE_CORPSE_COLLISION / 0x1B56E5A9 {
            entity: int,
        }
    }

    rpc! {
        /// sync_galactic_war_planet_event:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `arg2`: int, 3 bits, 0..7
        /// - `arg3`: int, 3 bits, 0..7
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `unsigned_value_4`: int, 32 bits, 0..0
        /// - `arg6`: u64
        /// - `arg7`: u64
        /// - `unsigned_value_5`: int, 32 bits, 0..0
        /// - `arg9`: up to 4 x int, 32 bits, 0..0
        /// - `id_string32`: u32
        /// - `unsigned_value_6`: int, 32 bits, 0..0
        SyncGalacticWarPlanetEvent / SYNC_GALACTIC_WAR_PLANET_EVENT / 0x1BE7CF6F {
            unsigned_value: int,
            unsigned_value_2: int,
            arg2: int,
            arg3: int,
            unsigned_value_3: int,
            unsigned_value_4: int,
            arg6: u64,
            arg7: u64,
            unsigned_value_5: int,
            arg9: [int; 4],
            id_string32: u32,
            unsigned_value_6: int,
        }
    }

    rpc! {
        /// loadout_set_stratagem_in_slot:
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `id_string32`: u32
        LoadoutSetStratagemInSlot / LOADOUT_SET_STRATAGEM_IN_SLOT / 0x1C826120 {
            entity: int,
            unsigned_value: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// return_to_mission_briefing (session-bound): no arguments.
        ReturnToMissionBriefing / RETURN_TO_MISSION_BRIEFING / 0x1D24BC8A;
    }

    rpc! {
        /// animation_variable_int (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `int_value`: int, 32 bits, 0..0
        AnimationVariableInt / ANIMATION_VARIABLE_INT / 0x1DE327E3 {
            entity: int,
            unsigned_value: int,
            int_value: int,
        }
    }

    rpc! {
        /// equipment_slot_change_start_percent (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        EquipmentSlotChangeStartPercent / EQUIPMENT_SLOT_CHANGE_START_PERCENT / 0x1E275179 {
            entity: int,
            unsigned_value: int,
            float_value: float,
        }
    }

    rpc! {
        /// arm_mines (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ArmMines / ARM_MINES / 0x1E2AE313 {
            entity: int,
        }
    }

    rpc! {
        /// set_target_list (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 8 x int, 15 bits, 0..32767
        SetTargetList / SET_TARGET_LIST / 0x1EDE4867 {
            entity: int,
            arg1: [int; 8],
        }
    }

    rpc! {
        /// set_minigame_reels_selected_slot (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetMinigameReelsSelectedSlot / SET_MINIGAME_REELS_SELECTED_SLOT / 0x1F3DF90C {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// destructible_on_destroyed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `peer_id`: u64
        /// - `entity_2`: int, 15 bits, 0..32767
        DestructibleOnDestroyed / DESTRUCTIBLE_ON_DESTROYED / 0x1F4E13B0 {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            unsigned_value: int,
            unsigned_value_2: int,
            peer_id: u64,
            entity_2: int,
        }
    }

    rpc! {
        /// request_loadout_sync: no arguments.
        RequestLoadoutSync / REQUEST_LOADOUT_SYNC / 0x1F51A605;
    }

    rpc! {
        /// run_ability (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 12 bits, 0..4095
        /// - `arg2`: float, 10 bits over 0..20
        RunAbility / RUN_ABILITY / 0x1FB93C3B {
            entity: int,
            arg1: int,
            arg2: float,
        }
    }

    rpc! {
        /// set_cape_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 2 bits, 0..3
        SetCapeState / SET_CAPE_STATE / 0x206B7867 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// on_killed_by_player (session-bound): no arguments.
        OnKilledByPlayer / ON_KILLED_BY_PLAYER / 0x2084CB8E;
    }

    rpc! {
        /// activate_explosive_variable_timer (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        ActivateExplosiveVariableTimer / ACTIVATE_EXPLOSIVE_VARIABLE_TIMER / 0x20D3D1A6 {
            entity: int,
            bool_value: bool,
            float_value: float,
        }
    }

    rpc! {
        /// apply_momentum (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        ApplyMomentum / APPLY_MOMENTUM / 0x211A5AFE {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
        }
    }

    rpc! {
        /// try_add_corpse (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        TryAddCorpse / TRY_ADD_CORPSE / 0x21666C32 {
            entity: int,
        }
    }

    rpc! {
        /// arcade_stratagem_lost_combo (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ArcadeStratagemLostCombo / ARCADE_STRATAGEM_LOST_COMBO / 0x21B6D7F4 {
            entity: int,
        }
    }

    rpc! {
        /// destroy_unit (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 3 bits, 0..7
        DestroyUnit / DESTROY_UNIT / 0x220F970F {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// play_animation_event_if_cutscene (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        PlayAnimationEventIfCutscene / PLAY_ANIMATION_EVENT_IF_CUTSCENE / 0x224FABB1 {
            entity: int,
            id_string32: u32,
            unsigned_value: int,
        }
    }

    rpc! {
        /// sync_galactic_war_global_resource:
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        SyncGalacticWarGlobalResource / SYNC_GALACTIC_WAR_GLOBAL_RESOURCE / 0x2320CF16 {
            id_string32: u32,
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
            float_value: float,
        }
    }

    rpc! {
        /// stop_follow_curve (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        StopFollowCurve / STOP_FOLLOW_CURVE / 0x2354D786 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// set_spawn_cooldown_override (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        SetSpawnCooldownOverride / SET_SPAWN_COOLDOWN_OVERRIDE / 0x2419DC20 {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// do_explosion (session-bound):
        ///
        /// Arguments:
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `peer_id`: u64
        /// - `entity_3`: int, 15 bits, 0..32767
        DoExplosion / DO_EXPLOSION / 0x24A7BFCB {
            vector3: vector3,
            unsigned_value: int,
            entity: int,
            entity_2: int,
            peer_id: u64,
            entity_3: int,
        }
    }

    rpc! {
        /// convoy_left_mission_area (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ConvoyLeftMissionArea / CONVOY_LEFT_MISSION_AREA / 0x256A335A {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// set_switch_state_input (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 2 bits, 0..3
        SetSwitchStateInput / SET_SWITCH_STATE_INPUT / 0x25DA70D0 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// detach (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        Detach / DETACH / 0x25DE2F8B {
            entity: int,
        }
    }

    rpc! {
        /// request_exit_seat_collection (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `seat_index`: int, 5 bits, -1..30
        /// - `bool_value`: bool
        RequestExitSeatCollection / REQUEST_EXIT_SEAT_COLLECTION / 0x260F8167 {
            entity: int,
            entity_2: int,
            seat_index: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// wield_weapon (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        /// - `entity_2`: int, 15 bits, 0..32767
        WieldWeapon / WIELD_WEAPON / 0x2671DEC5 {
            entity: int,
            arg1: int,
            entity_2: int,
        }
    }

    rpc! {
        /// finalize_avatar_corpse (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        FinalizeAvatarCorpse / FINALIZE_AVATAR_CORPSE / 0x268A90D4 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// stop_approach (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        StopApproach / STOP_APPROACH / 0x272E7702 {
            entity: int,
        }
    }

    rpc! {
        /// set_collision_filter (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        SetCollisionFilter / SET_COLLISION_FILTER / 0x27B5D265 {
            entity: int,
        }
    }

    rpc! {
        /// sync_galactic_war_selection:
        ///
        /// Arguments:
        /// - `int_value`: int, 32 bits, 0..0
        /// - `int_value_2`: int, 32 bits, 0..0
        /// - `int_value_3`: int, 32 bits, 0..0
        /// - `int_value_4`: int, 32 bits, 0..0
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `int_value_5`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        SyncGalacticWarSelection / SYNC_GALACTIC_WAR_SELECTION / 0x27C74B16 {
            int_value: int,
            int_value_2: int,
            int_value_3: int,
            int_value_4: int,
            unsigned_value: int,
            int_value_5: int,
            bool_value: bool,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// from_host_respawn (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `entity_3`: int, 15 bits, 0..32767
        /// - `bool_value_2`: bool
        FromHostRespawn / FROM_HOST_RESPAWN / 0x280728D6 {
            entity: int,
            vector3: vector3,
            entity_2: int,
            bool_value: bool,
            entity_3: int,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// sync_attachment (session-bound):
        ///
        /// Arguments:
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        SyncAttachment / SYNC_ATTACHMENT / 0x282AED7C {
            vector3: vector3,
            quaternion: quaternion,
        }
    }

    rpc! {
        /// heal_all_injuries (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        HealAllInjuries / HEAL_ALL_INJURIES / 0x283BAAE5 {
            entity: int,
        }
    }

    rpc! {
        /// loadout_set_equipment_in_slot (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `id_string32`: u32
        LoadoutSetEquipmentInSlot / LOADOUT_SET_EQUIPMENT_IN_SLOT / 0x286A772D {
            entity: int,
            unsigned_value: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// request_mark_squad (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `entity_3`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        RequestMarkSquad / REQUEST_MARK_SQUAD / 0x287A2554 {
            entity: int,
            entity_2: int,
            entity_3: int,
            float_value: float,
        }
    }

    rpc! {
        /// revert_to_switch_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        RevertToSwitchState / REVERT_TO_SWITCH_STATE / 0x28F745E7 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// dialogue_playing (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        DialoguePlaying / DIALOGUE_PLAYING / 0x2923EDEF {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// score_sync_unsigned (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        ScoreSyncUnsigned / SCORE_SYNC_UNSIGNED / 0x2994C3D9 {
            id_string32: u32,
            unsigned_value: int,
        }
    }

    rpc! {
        /// request_score_sync (session-bound): no arguments.
        RequestScoreSync / REQUEST_SCORE_SYNC / 0x2999705A;
    }

    rpc! {
        /// terrain_deformation_event_list:
        ///
        /// Arguments:
        /// - `arg0`: up to 64 x u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        TerrainDeformationEventList / TERRAIN_DEFORMATION_EVENT_LIST / 0x2A47A190 {
            arg0: [u64; 64],
            unsigned_value: int,
        }
    }

    rpc! {
        /// set_ship_name (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `network_string`: string, at most 2047 bytes
        SetShipName / SET_SHIP_NAME / 0x2AD4AE42 {
            entity: int,
            network_string: string,
        }
    }

    rpc! {
        /// player_ship_module_synced_data:
        ///
        /// Arguments:
        /// - `arg0`: up to 2 x u64
        PlayerShipModuleSyncedData / PLAYER_SHIP_MODULE_SYNCED_DATA / 0x2AD79D8F {
            arg0: [u64; 2],
        }
    }

    rpc! {
        /// spawn_unit (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 3 bits, 0..7
        SpawnUnit / SPAWN_UNIT / 0x2BC6739E {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// set_cape_enabled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetCapeEnabled / SET_CAPE_ENABLED / 0x2BEC9351 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// set_held_carryable (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 2 bits, 0..3
        SetHeldCarryable / SET_HELD_CARRYABLE / 0x2C4D5D64 {
            entity: int,
            entity_2: int,
            arg2: int,
        }
    }

    rpc! {
        /// reset_loaded_loadout: no arguments.
        ResetLoadedLoadout / RESET_LOADED_LOADOUT / 0x2D295D18;
    }

    rpc! {
        /// from_client_synced_task_abort: no arguments.
        FromClientSyncedTaskAbort / FROM_CLIENT_SYNCED_TASK_ABORT / 0x2D5B6605;
    }

    rpc! {
        /// prepare_avatar_corpse (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        PrepareAvatarCorpse / PREPARE_AVATAR_CORPSE / 0x2DAC0E51 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// request_dialogue (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        RequestDialogue / REQUEST_DIALOGUE / 0x2E7FC495 {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// request_seat_success (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `seat_index`: int, 5 bits, -1..30
        RequestSeatSuccess / REQUEST_SEAT_SUCCESS / 0x2E986F01 {
            entity: int,
            entity_2: int,
            seat_index: int,
        }
    }

    rpc! {
        /// from_migration_host_missing_events (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 64 x u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        FromMigrationHostMissingEvents / FROM_MIGRATION_HOST_MISSING_EVENTS / 0x30ED613C {
            arg0: [u64; 64],
            unsigned_value: int,
        }
    }

    rpc! {
        /// recharge_activate (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        RechargeActivate / RECHARGE_ACTIVATE / 0x30EFE856 {
            entity: int,
        }
    }

    rpc! {
        /// set_aiming_weapon (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        SetAimingWeapon / SET_AIMING_WEAPON / 0x31B2BC44 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// do_avatar_pain_exertion (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        DoAvatarPainExertion / DO_AVATAR_PAIN_EXERTION / 0x327F7938 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// reinforcement_cooldown (session-bound):
        ///
        /// Arguments:
        /// - `float_value`: float, 31 bits over -10000..10000
        ReinforcementCooldown / REINFORCEMENT_COOLDOWN / 0x3293DFA6 {
            float_value: float,
        }
    }

    rpc! {
        /// from_client_request_loadout_weapon_swap (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `id_string32`: u32
        /// - `arg3`: up to 48 x int, 32 bits, 0..0
        /// - `arg4`: up to 48 x int, 32 bits, 0..0
        FromClientRequestLoadoutWeaponSwap / FROM_CLIENT_REQUEST_LOADOUT_WEAPON_SWAP / 0x32DABA7A {
            arg0: u64,
            unsigned_value: int,
            id_string32: u32,
            arg3: [int; 48],
            arg4: [int; 48],
        }
    }

    rpc! {
        /// request_hellpad_confirmation (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 8 bits, 0..255
        RequestHellpadConfirmation / REQUEST_HELLPAD_CONFIRMATION / 0x33045DF8 {
            arg0: int,
        }
    }

    rpc! {
        /// request_mission_over (session-bound): no arguments.
        RequestMissionOver / REQUEST_MISSION_OVER / 0x3330AB9F;
    }

    rpc! {
        /// set_animation_states (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 3 x int, 32 bits, 0..0
        SetAnimationStates / SET_ANIMATION_STATES / 0x34B17419 {
            entity: int,
            arg1: [int; 3],
        }
    }

    rpc! {
        /// share_target (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `id_string32`: u32
        ShareTarget / SHARE_TARGET / 0x3588F5A1 {
            entity: int,
            vector3: vector3,
            entity_2: int,
            vector3_2: vector3,
            float_value: float,
            unsigned_value: int,
            float_value_2: float,
            id_string32: u32,
        }
    }

    rpc! {
        /// from_partyleader_join_request:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: up to 5 x u64
        /// - `id_string64`: id64, a name hash
        /// - `id_string64_2`: id64, a name hash
        /// - `arg4`: int, 3 bits, 0..7
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        FromPartyleaderJoinRequest / FROM_PARTYLEADER_JOIN_REQUEST / 0x359461FE {
            arg0: u64,
            arg1: [u64; 5],
            id_string64: id64,
            id_string64_2: id64,
            arg4: int,
            bool_value: bool,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// request_hostability (session-bound): no arguments.
        RequestHostability / REQUEST_HOSTABILITY / 0x36334D22;
    }

    rpc! {
        /// store_entity (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        StoreEntity / STORE_ENTITY / 0x367D8909 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// start_status_effect_damage (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: int, 7 bits, 0..127
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `peer_id`: u64
        StartStatusEffectDamage / START_STATUS_EFFECT_DAMAGE / 0x368A8A25 {
            entity: int,
            unsigned_value: int,
            arg2: int,
            entity_2: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// set_minigame_pipe_slide_selected_index (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetMinigamePipeSlideSelectedIndex / SET_MINIGAME_PIPE_SLIDE_SELECTED_INDEX / 0x36EEFD92 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// explode_after_delay (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        ExplodeAfterDelay / EXPLODE_AFTER_DELAY / 0x37219C41 {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// on_minigame_press_any_button_pressed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        OnMinigamePressAnyButtonPressed / ON_MINIGAME_PRESS_ANY_BUTTON_PRESSED / 0x377902BF {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// from_client_ready_for_network_play (session-bound):
        ///
        /// Arguments:
        /// - `bool_value`: bool
        FromClientReadyForNetworkPlay / FROM_CLIENT_READY_FOR_NETWORK_PLAY / 0x37FF1AA7 {
            bool_value: bool,
        }
    }

    rpc! {
        /// sync_attachment_full (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        SyncAttachmentFull / SYNC_ATTACHMENT_FULL / 0x38099456 {
            entity: int,
            unsigned_value: int,
            vector3: vector3,
            quaternion: quaternion,
        }
    }

    rpc! {
        /// set_sticky_transform (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        SetStickyTransform / SET_STICKY_TRANSFORM / 0x3828593F {
            entity: int,
            vector3: vector3,
            quaternion: quaternion,
        }
    }

    rpc! {
        /// mission_progression_result_received (session-bound):
        ///
        /// Arguments:
        /// - `bool_value`: bool
        /// - `arg1`: up to 8 x int, 32 bits, 0..0
        /// - `arg2`: up to 8 x int, 32 bits, 0..0
        /// - `arg3`: up to 8 x int, 32 bits, 0..0
        /// - `arg4`: up to 8 x int, 32 bits, 0..0
        /// - `arg5`: up to 8 x int, 32 bits, 0..0
        /// - `arg6`: up to 15 x u64
        /// - `arg7`: up to 15 x int, 8 bits, 0..255
        /// - `arg8`: up to 15 x int, 8 bits, 0..255
        /// - `arg9`: up to 15 x int, 16 bits, 0..65535
        /// - `network_string`: string, at most 2047 bytes
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `bool_value_2`: bool
        /// - `arg14`: up to 4 x int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `arg16`: up to 4 x int, 32 bits, 0..0
        /// - `arg17`: up to 4 x float, 31 bits over -10000..10000
        /// - `arg18`: up to 4 x u64
        MissionProgressionResultReceived / MISSION_PROGRESSION_RESULT_RECEIVED / 0x392EB78D {
            bool_value: bool,
            arg1: [int; 8],
            arg2: [int; 8],
            arg3: [int; 8],
            arg4: [int; 8],
            arg5: [int; 8],
            arg6: [u64; 15],
            arg7: [int; 15],
            arg8: [int; 15],
            arg9: [int; 15],
            network_string: string,
            unsigned_value: int,
            unsigned_value_2: int,
            bool_value_2: bool,
            arg14: [int; 4],
            unsigned_value_3: int,
            arg16: [int; 4],
            arg17: [float; 4],
            arg18: [u64; 4],
        }
    }

    rpc! {
        /// request_enter_seat_collection (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 4 bits, 0..15
        RequestEnterSeatCollection / REQUEST_ENTER_SEAT_COLLECTION / 0x3A44E090 {
            entity: int,
            entity_2: int,
            arg2: int,
        }
    }

    rpc! {
        /// activate_explosive (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        ActivateExplosive / ACTIVATE_EXPLOSIVE / 0x3A6F7DAE {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// start_approach (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `vector3_3`: vector3, 31 bits over -10000..10000
        StartApproach / START_APPROACH / 0x3AC647FB {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            vector3_3: vector3,
        }
    }

    rpc! {
        /// ability_target (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        AbilityTarget / ABILITY_TARGET / 0x3B37AAE6 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// hotjoin_ability_target (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `position`: vector3, 17 bits over -2000..2000
        HotjoinAbilityTarget / HOTJOIN_ABILITY_TARGET / 0x3B8A137D {
            entity: int,
            entity_2: int,
            position: vector3,
        }
    }

    rpc! {
        /// sync_debug_time_of_day (session-bound):
        ///
        /// Arguments:
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `float_value_3`: float, 31 bits over -10000..10000
        /// - `int_value`: int, 32 bits, 0..0
        /// - `int_value_2`: int, 32 bits, 0..0
        /// - `float_value_4`: float, 31 bits over -10000..10000
        /// - `float_value_5`: float, 31 bits over -10000..10000
        /// - `float_value_6`: float, 31 bits over -10000..10000
        SyncDebugTimeOfDay / SYNC_DEBUG_TIME_OF_DAY / 0x3B9DCDA9 {
            float_value: float,
            float_value_2: float,
            float_value_3: float,
            int_value: int,
            int_value_2: int,
            float_value_4: float,
            float_value_5: float,
            float_value_6: float,
        }
    }

    rpc! {
        /// host_abandoned_mission (session-bound): no arguments.
        HostAbandonedMission / HOST_ABANDONED_MISSION / 0x3BF5AECF;
    }

    rpc! {
        /// explosive_entered_basket (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        ExplosiveEnteredBasket / EXPLOSIVE_ENTERED_BASKET / 0x3C6BCD0C {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// from_partymember_return_rejected: no arguments.
        FromPartymemberReturnRejected / FROM_PARTYMEMBER_RETURN_REJECTED / 0x3CB3C266;
    }

    rpc! {
        /// debug_force_load_package:
        ///
        /// Arguments:
        /// - `arg0`: u64
        DebugForceLoadPackage / DEBUG_FORCE_LOAD_PACKAGE / 0x3CC5F10A {
            arg0: u64,
        }
    }

    rpc! {
        /// objective_discovered (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ObjectiveDiscovered / OBJECTIVE_DISCOVERED / 0x3CFB4B33 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// peer_wants_to_leave: no arguments.
        PeerWantsToLeave / PEER_WANTS_TO_LEAVE / 0x3D1187C7;
    }

    rpc! {
        /// on_assignment_game_over (session-bound): no arguments.
        OnAssignmentGameOver / ON_ASSIGNMENT_GAME_OVER / 0x3D9FF3EA;
    }

    rpc! {
        /// inform_clients_of_impact (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        InformClientsOfImpact / INFORM_CLIENTS_OF_IMPACT / 0x3DCCBD87 {
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// apply_impulse (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        ApplyImpulse / APPLY_IMPULSE / 0x3E3F525A {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// from_host_party_join_denied:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: int, 5 bits, 0..31
        /// - `arg2`: u64
        FromHostPartyJoinDenied / FROM_HOST_PARTY_JOIN_DENIED / 0x3E41A4B8 {
            arg0: u64,
            arg1: int,
            arg2: u64,
        }
    }

    rpc! {
        /// set_targeting_constraint_locked (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetTargetingConstraintLocked / SET_TARGETING_CONSTRAINT_LOCKED / 0x3E744D68 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// request_play_animation_event_synced (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        RequestPlayAnimationEventSynced / REQUEST_PLAY_ANIMATION_EVENT_SYNCED / 0x407A4429 {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// nag_for_respawn (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        NagForRespawn / NAG_FOR_RESPAWN / 0x40BD50CC {
            entity: int,
        }
    }

    rpc! {
        /// debug_force_ready_up (session-bound): no arguments.
        DebugForceReadyUp / DEBUG_FORCE_READY_UP / 0x41A6D1FB;
    }

    rpc! {
        /// unwield_weapon (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        UnwieldWeapon / UNWIELD_WEAPON / 0x423A4034 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// set_inventory (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `entity_3`: int, 15 bits, 0..32767
        /// - `entity_4`: int, 15 bits, 0..32767
        /// - `entity_5`: int, 15 bits, 0..32767
        /// - `entity_6`: int, 15 bits, 0..32767
        /// - `id_string64`: id64, a name hash
        SetInventory / SET_INVENTORY / 0x42729286 {
            entity: int,
            entity_2: int,
            entity_3: int,
            entity_4: int,
            entity_5: int,
            entity_6: int,
            id_string64: id64,
        }
    }

    rpc! {
        /// on_force_to_loadout (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        OnForceToLoadout / ON_FORCE_TO_LOADOUT / 0x42F5D99F {
            entity: int,
        }
    }

    rpc! {
        /// set_countdown_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetCountdownState / SET_COUNTDOWN_STATE / 0x4326B4D3 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// trigger_ship_game_event (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        TriggerShipGameEvent / TRIGGER_SHIP_GAME_EVENT / 0x433A8DC0 {
            id_string32: u32,
        }
    }

    rpc! {
        /// from_client_request_loadout_load:
        ///
        /// Arguments:
        /// - `bool_value`: bool
        FromClientRequestLoadoutLoad / FROM_CLIENT_REQUEST_LOADOUT_LOAD / 0x4391C0FF {
            bool_value: bool,
        }
    }

    rpc! {
        /// talk_with_target_about (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        TalkWithTargetAbout / TALK_WITH_TARGET_ABOUT / 0x43D97EC6 {
            entity: int,
            entity_2: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// set_avatar_ik_enabled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        SetAvatarIkEnabled / SET_AVATAR_IK_ENABLED / 0x444EF89A {
            entity: int,
            bool_value: bool,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// sync_stratagem_changes (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 16 x int, 32 bits, 0..0
        /// - `arg1`: up to 16 x int, 32 bits, 0..0
        /// - `arg2`: up to 16 x u64
        /// - `arg3`: up to 32 x int, 32 bits, 0..0
        /// - `arg4`: up to 16 x bool
        SyncStratagemChanges / SYNC_STRATAGEM_CHANGES / 0x44548DC7 {
            arg0: [int; 16],
            arg1: [int; 16],
            arg2: [u64; 16],
            arg3: [int; 32],
            arg4: [bool; 16],
        }
    }

    rpc! {
        /// sync_hive_lord_killed (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        SyncHiveLordKilled / SYNC_HIVE_LORD_KILLED / 0x445B2BBE {
            unsigned_value: int,
        }
    }

    rpc! {
        /// trigger_ui_audio_event (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        TriggerUiAudioEvent / TRIGGER_UI_AUDIO_EVENT / 0x446AB9AB {
            id_string32: u32,
        }
    }

    rpc! {
        /// remove_interested_dialogue_target (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `peer_id`: u64
        RemoveInterestedDialogueTarget / REMOVE_INTERESTED_DIALOGUE_TARGET / 0x454E8B9C {
            entity: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// set_countdown_time_index (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetCountdownTimeIndex / SET_COUNTDOWN_TIME_INDEX / 0x457DFF9A {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// restore_limbs (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        RestoreLimbs / RESTORE_LIMBS / 0x45D32529 {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// on_commend_count_received (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        OnCommendCountReceived / ON_COMMEND_COUNT_RECEIVED / 0x473AFAFF {
            unsigned_value: int,
        }
    }

    rpc! {
        /// begin_dropin (session-bound): no arguments.
        BeginDropin / BEGIN_DROPIN / 0x47575E6B;
    }

    rpc! {
        /// get_ingame_ship_timeline (session-bound):
        ///
        /// Arguments:
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `float_value_3`: float, 31 bits over -10000..10000
        /// - `float_value_4`: float, 31 bits over -10000..10000
        GetIngameShipTimeline / GET_INGAME_SHIP_TIMELINE / 0x478CDB8E {
            float_value: float,
            float_value_2: float,
            float_value_3: float,
            float_value_4: float,
        }
    }

    rpc! {
        /// from_host_peer_confirmed:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `peer_id_2`: u64
        /// - `arg2`: int, 3 bits, 0..7
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        FromHostPeerConfirmed / FROM_HOST_PEER_CONFIRMED / 0x4804FCF4 {
            peer_id: u64,
            peer_id_2: u64,
            arg2: int,
            unsigned_value: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// loadout_select_stratagem_slot (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        LoadoutSelectStratagemSlot / LOADOUT_SELECT_STRATAGEM_SLOT / 0x4924EA3C {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// trigger_ui_audio_event_from_drop_menu_3d (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        TriggerUiAudioEventFromDropMenu3d / TRIGGER_UI_AUDIO_EVENT_FROM_DROP_MENU_3D / 0x495376DB {
            id_string32: u32,
        }
    }

    rpc! {
        /// debug_set_game_session_host (session-bound): no arguments.
        DebugSetGameSessionHost / DEBUG_SET_GAME_SESSION_HOST / 0x4A881B79;
    }

    rpc! {
        /// add_teammate_needs_reload_marker (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        AddTeammateNeedsReloadMarker / ADD_TEAMMATE_NEEDS_RELOAD_MARKER / 0x4A8E79BA {
            entity: int,
        }
    }

    rpc! {
        /// request_exit_seat_collection_success (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        RequestExitSeatCollectionSuccess / REQUEST_EXIT_SEAT_COLLECTION_SUCCESS / 0x4AD5AE34 {
            entity: int,
            entity_2: int,
            unsigned_value: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// physics_local_impulse (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        PhysicsLocalImpulse / PHYSICS_LOCAL_IMPULSE / 0x4AF4C703 {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// debug_apply_existing_customization (session-bound): no arguments.
        DebugApplyExistingCustomization / DEBUG_APPLY_EXISTING_CUSTOMIZATION / 0x4AFFE835;
    }

    rpc! {
        /// ability_request_denied (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        AbilityRequestDenied / ABILITY_REQUEST_DENIED / 0x4B4E43EC {
            entity: int,
        }
    }

    rpc! {
        /// from_host_partymember_join_pre_confirmed:
        ///
        /// Arguments:
        /// - `arg0`: u64
        FromHostPartymemberJoinPreConfirmed / FROM_HOST_PARTYMEMBER_JOIN_PRE_CONFIRMED / 0x4BFB3315 {
            arg0: u64,
        }
    }

    rpc! {
        /// set_temperature_loss_modifier (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        SetTemperatureLossModifier / SET_TEMPERATURE_LOSS_MODIFIER / 0x4CEC5BB6 {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// sync_unit_customization:
        ///
        /// Arguments:
        /// - `arg0`: up to 12 x int, 32 bits, 0..0
        SyncUnitCustomization / SYNC_UNIT_CUSTOMIZATION / 0x4D38EF84 {
            arg0: [int; 12],
        }
    }

    rpc! {
        /// hotjoin_ability (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 12 bits, 0..4095
        /// - `ability_frame`: int, 13 bits, 0..8191
        /// - `unsigned_value`: int, 32 bits, 0..0
        HotjoinAbility / HOTJOIN_ABILITY / 0x4DBC9D2F {
            entity: int,
            arg1: int,
            ability_frame: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// set_avatar_visibility (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        SetAvatarVisibility / SET_AVATAR_VISIBILITY / 0x4DC34BEE {
            entity: int,
            bool_value: bool,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// sync_avatar_death_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        SyncAvatarDeathState / SYNC_AVATAR_DEATH_STATE / 0x4EBDD4AE {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            unsigned_value: int,
            float_value: float,
        }
    }

    rpc! {
        /// set_active_and_retract (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        SetActiveAndRetract / SET_ACTIVE_AND_RETRACT / 0x4ECE32BD {
            entity: int,
            bool_value: bool,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// spawn_extraction_beacon_sfx (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        SpawnExtractionBeaconSfx / SPAWN_EXTRACTION_BEACON_SFX / 0x508C4BDE {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// from_host_party_join_accepted:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: up to 5 x u64
        FromHostPartyJoinAccepted / FROM_HOST_PARTY_JOIN_ACCEPTED / 0x50C79002 {
            arg0: u64,
            arg1: [u64; 5],
        }
    }

    rpc! {
        /// interaction_request_result (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 4 bits, 0..15
        /// - `entity_3`: int, 15 bits, 0..32767
        /// - `arg4`: int, 1 bits, 0..1
        InteractionRequestResult / INTERACTION_REQUEST_RESULT / 0x50DFB054 {
            entity: int,
            entity_2: int,
            arg2: int,
            entity_3: int,
            arg4: int,
        }
    }

    rpc! {
        /// from_partymember_cancel_join:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `peer_id`: u64
        /// - `arg2`: int, 2 bits, 0..3
        FromPartymemberCancelJoin / FROM_PARTYMEMBER_CANCEL_JOIN / 0x5133060D {
            arg0: u64,
            peer_id: u64,
            arg2: int,
        }
    }

    rpc! {
        /// hostability (session-bound):
        ///
        /// Arguments:
        /// - `float_value`: float, 31 bits over -10000..10000
        Hostability / HOSTABILITY / 0x520227A4 {
            float_value: float,
        }
    }

    rpc! {
        /// from_host_partymember_join_denied:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: int, 5 bits, 0..31
        /// - `arg2`: u64
        FromHostPartymemberJoinDenied / FROM_HOST_PARTYMEMBER_JOIN_DENIED / 0x521CC34F {
            arg0: u64,
            arg1: int,
            arg2: u64,
        }
    }

    rpc! {
        /// hotjoin_ability_pending_ability (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 12 bits, 0..4095
        HotjoinAbilityPendingAbility / HOTJOIN_ABILITY_PENDING_ABILITY / 0x5369ECFF {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// ability_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 12 bits, 0..4095
        AbilityRequest / ABILITY_REQUEST / 0x549D8B99 {
            entity: int,
            entity_2: int,
            arg2: int,
        }
    }

    rpc! {
        /// sync_game_state:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        SyncGameState / SYNC_GAME_STATE / 0x54B55810 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// loot_container_pick_up_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        LootContainerPickUpRequest / LOOT_CONTAINER_PICK_UP_REQUEST / 0x55804AFA {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// play_cutscene (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg1`: int, 3 bits, 0..7
        PlayCutscene / PLAY_CUTSCENE / 0x569D19AD {
            unsigned_value: int,
            arg1: int,
        }
    }

    rpc! {
        /// set_transport_payload_cursor (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        SetTransportPayloadCursor / SET_TRANSPORT_PAYLOAD_CURSOR / 0x56AB955E {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// propose_drop_point (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        ProposeDropPoint / PROPOSE_DROP_POINT / 0x580ACBC7 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// set_zone_enabled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        /// - `bool_value`: bool
        SetZoneEnabled / SET_ZONE_ENABLED / 0x582E8E85 {
            entity: int,
            arg1: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// physics_uniform_impulse (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        PhysicsUniformImpulse / PHYSICS_UNIFORM_IMPULSE / 0x5845F11C {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// score_sync_bool (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        ScoreSyncBool / SCORE_SYNC_BOOL / 0x592F0D74 {
            id_string32: u32,
            unsigned_value: int,
        }
    }

    rpc! {
        /// score_sync_weapon_unsigned (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `id_string64`: id64, a name hash
        /// - `unsigned_value`: int, 32 bits, 0..0
        ScoreSyncWeaponUnsigned / SCORE_SYNC_WEAPON_UNSIGNED / 0x59F01A31 {
            id_string32: u32,
            id_string64: id64,
            unsigned_value: int,
        }
    }

    rpc! {
        /// remove_encounters_within (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 3 bits, 0..7
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `float_value`: float, 31 bits over -10000..10000
        RemoveEncountersWithin / REMOVE_ENCOUNTERS_WITHIN / 0x5ABB091E {
            arg0: int,
            vector3: vector3,
            float_value: float,
        }
    }

    rpc! {
        /// hellpad_confirmation_result (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 8 bits, 0..255
        /// - `bool_value`: bool
        HellpadConfirmationResult / HELLPAD_CONFIRMATION_RESULT / 0x5AC3BB74 {
            arg0: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// set_level_restarting (session-bound): no arguments.
        SetLevelRestarting / SET_LEVEL_RESTARTING / 0x5B79489D;
    }

    rpc! {
        /// change_difficulty_terrain (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        ChangeDifficultyTerrain / CHANGE_DIFFICULTY_TERRAIN / 0x5BB5A2D9 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// validate_exploration (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `vector3`: vector3, 31 bits over -10000..10000
        ValidateExploration / VALIDATE_EXPLORATION / 0x5C03D69C {
            arg0: u64,
            vector3: vector3,
        }
    }

    rpc! {
        /// objective_asset_destroyed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `damage_type`: int, 4 bits, 0..15
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `entity_3`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        ObjectiveAssetDestroyed / OBJECTIVE_ASSET_DESTROYED / 0x5C75F850 {
            entity: int,
            damage_type: int,
            entity_2: int,
            entity_3: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// from_partymember_join_failed:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: int, 5 bits, 0..31
        FromPartymemberJoinFailed / FROM_PARTYMEMBER_JOIN_FAILED / 0x5CBFD4A2 {
            arg0: u64,
            arg1: int,
        }
    }

    rpc! {
        /// material_variable_vector3 (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `float_value_3`: float, 31 bits over -10000..10000
        MaterialVariableVector3 / MATERIAL_VARIABLE_VECTOR3 / 0x5CCBCDC8 {
            entity: int,
            arg1: int,
            float_value: float,
            float_value_2: float,
            float_value_3: float,
        }
    }

    rpc! {
        /// entity_broken_command (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        EntityBrokenCommand / ENTITY_BROKEN_COMMAND / 0x5CE26B68 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// loadout_set_booster_id (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        LoadoutSetBoosterId / LOADOUT_SET_BOOSTER_ID / 0x5D509124 {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// trigger_hologram_audio_event (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        TriggerHologramAudioEvent / TRIGGER_HOLOGRAM_AUDIO_EVENT / 0x5D8A5D92 {
            id_string32: u32,
        }
    }

    rpc! {
        /// sync_galactic_war_planet_status:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `arg2`: int, 3 bits, 0..7
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `arg7`: up to 8 x int, 32 bits, 0..0
        /// - `arg8`: up to 32 x int, 32 bits, 0..0
        /// - `arg9`: int, 2 bits, 0..3
        SyncGalacticWarPlanetStatus / SYNC_GALACTIC_WAR_PLANET_STATUS / 0x5DCD723A {
            unsigned_value: int,
            vector3: vector3,
            arg2: int,
            unsigned_value_2: int,
            float_value: float,
            bool_value: bool,
            unsigned_value_3: int,
            arg7: [int; 8],
            arg8: [int; 32],
            arg9: int,
        }
    }

    rpc! {
        /// request_deform_terrain (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        RequestDeformTerrain / REQUEST_DEFORM_TERRAIN / 0x5E498407 {
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// sync_party_clan_station (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `int_value`: int, 32 bits, 0..0
        SyncPartyClanStation / SYNC_PARTY_CLAN_STATION / 0x5E5EF348 {
            unsigned_value: int,
            unsigned_value_2: int,
            int_value: int,
        }
    }

    rpc! {
        /// heal (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        Heal / HEAL / 0x5FCC6862 {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// displacement_start (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DisplacementStart / DISPLACEMENT_START / 0x601F79CB {
            entity: int,
        }
    }

    rpc! {
        /// landing_started (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        LandingStarted / LANDING_STARTED / 0x60A8100F {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// set_weapon_flashlight_mode (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `flashlight_mode`: int, 2 bits, 0..3
        SetWeaponFlashlightMode / SET_WEAPON_FLASHLIGHT_MODE / 0x61EC4B6E {
            entity: int,
            flashlight_mode: int,
        }
    }

    rpc! {
        /// set_stance (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `stance`: int, 2 bits, 0..3
        SetStance / SET_STANCE / 0x621CD409 {
            entity: int,
            stance: int,
        }
    }

    rpc! {
        /// equipment_slot_change_start (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        EquipmentSlotChangeStart / EQUIPMENT_SLOT_CHANGE_START / 0x626F62E7 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// from_host_partymember_join_confirmed:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `network_string`: string, at most 2047 bytes
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg3`: u64
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `arg5`: int, 3 bits, 0..7
        /// - `arg6`: int, 2 bits, 0..3
        /// - `arg7`: int, 3 bits, 0..7
        /// - `client_rejoin_state`: int, 2 bits, 0..3
        FromHostPartymemberJoinConfirmed / FROM_HOST_PARTYMEMBER_JOIN_CONFIRMED / 0x62B8A2CD {
            arg0: u64,
            network_string: string,
            unsigned_value: int,
            arg3: u64,
            vector3: vector3,
            arg5: int,
            arg6: int,
            arg7: int,
            client_rejoin_state: int,
        }
    }

    rpc! {
        /// dialogue_finished (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        DialogueFinished / DIALOGUE_FINISHED / 0x638C3A6F {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// from_host_join_failed: no arguments.
        FromHostJoinFailed / FROM_HOST_JOIN_FAILED / 0x63C5F439;
    }

    rpc! {
        /// dyndest_on_hotjoined (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 11 bits, 0..2047
        /// - `arg1`: u64
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        DyndestOnHotjoined / DYNDEST_ON_HOTJOINED / 0x64112598 {
            arg0: int,
            arg1: u64,
            vector3: vector3,
            quaternion: quaternion,
        }
    }

    rpc! {
        /// from_partyleader_cancel_join:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: int, 2 bits, 0..3
        FromPartyleaderCancelJoin / FROM_PARTYLEADER_CANCEL_JOIN / 0x6450B866 {
            arg0: u64,
            arg1: int,
        }
    }

    rpc! {
        /// from_migration_host_done (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        FromMigrationHostDone / FROM_MIGRATION_HOST_DONE / 0x64BA50A5 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// play_chained (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 8 x u32
        /// - `arg1`: up to 8 x float, 31 bits over -10000..10000
        /// - `arg2`: up to 8 x u32
        /// - `arg3`: up to 8 x int, 32 bits, 0..0
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        PlayChained / PLAY_CHAINED / 0x64E7A62A {
            arg0: [u32; 8],
            arg1: [float; 8],
            arg2: [u32; 8],
            arg3: [int; 8],
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// animation_variable (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        AnimationVariable / ANIMATION_VARIABLE / 0x655889B0 {
            entity: int,
            unsigned_value: int,
            float_value: float,
        }
    }

    rpc! {
        /// sync_stratagem_shared_uses_left:
        ///
        /// Arguments:
        /// - `arg0`: up to 16 x int, 32 bits, 0..0
        /// - `arg1`: up to 16 x int, 32 bits, 0..0
        SyncStratagemSharedUsesLeft / SYNC_STRATAGEM_SHARED_USES_LEFT / 0x659293F9 {
            arg0: [int; 16],
            arg1: [int; 16],
        }
    }

    rpc! {
        /// transport_drop (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        TransportDrop / TRANSPORT_DROP / 0x6812B5BE {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// displacement_teleport (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        DisplacementTeleport / DISPLACEMENT_TELEPORT / 0x688EAD64 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// loadout_select_equipment_slot (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        LoadoutSelectEquipmentSlot / LOADOUT_SELECT_EQUIPMENT_SLOT / 0x69358EBC {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// encounter_spawn_ping (session-bound):
        ///
        /// Arguments:
        /// - `vector3`: vector3, 31 bits over -10000..10000
        EncounterSpawnPing / ENCOUNTER_SPAWN_PING / 0x69791EB5 {
            vector3: vector3,
        }
    }

    rpc! {
        /// set_held_throwable (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetHeldThrowable / SET_HELD_THROWABLE / 0x6A24D979 {
            entity: int,
            entity_2: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// modifier_system_recieve_modifier_from_host (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg1`: u64
        ModifierSystemRecieveModifierFromHost / MODIFIER_SYSTEM_RECIEVE_MODIFIER_FROM_HOST / 0x6A832A0E {
            unsigned_value: int,
            arg1: u64,
        }
    }

    rpc! {
        /// hotjoin_ability_set_undo_mask (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: u64
        HotjoinAbilitySetUndoMask / HOTJOIN_ABILITY_SET_UNDO_MASK / 0x6A97A463 {
            entity: int,
            arg1: u64,
        }
    }

    rpc! {
        /// assisted_reload_partner_ready_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        AssistedReloadPartnerReadyState / ASSISTED_RELOAD_PARTNER_READY_STATE / 0x6AA803C2 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// animation_event_percent (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        AnimationEventPercent / ANIMATION_EVENT_PERCENT / 0x6AFC3074 {
            entity: int,
            unsigned_value: int,
            float_value: float,
        }
    }

    rpc! {
        /// heal_injury (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        HealInjury / HEAL_INJURY / 0x6B952293 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// hotjoin_info (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `int_value`: int, 32 bits, 0..0
        /// - `arg2`: up to 2 x u64
        HotjoinInfo / HOTJOIN_INFO / 0x6C80DEA8 {
            entity: int,
            int_value: int,
            arg2: [u64; 2],
        }
    }

    rpc! {
        /// animation_event_duration_percent (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `animation_event`: int, 13 bits, 0..8191
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        AnimationEventDurationPercent / ANIMATION_EVENT_DURATION_PERCENT / 0x6CA0876D {
            entity: int,
            animation_event: int,
            float_value: float,
            float_value_2: float,
            bool_value: bool,
        }
    }

    rpc! {
        /// pawn_deserting (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        PawnDeserting / PAWN_DESERTING / 0x6CD0CD37 {
            entity: int,
        }
    }

    rpc! {
        /// set_moving_hazard_time_remaining (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        SetMovingHazardTimeRemaining / SET_MOVING_HAZARD_TIME_REMAINING / 0x6EBEF84D {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// from_migration_host_event_count (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        FromMigrationHostEventCount / FROM_MIGRATION_HOST_EVENT_COUNT / 0x6ED6B2AE {
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// state_changed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 2 bits, 0..3
        StateChanged / STATE_CHANGED / 0x6F1A4AC6 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// weapon_level_up (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        WeaponLevelUp / WEAPON_LEVEL_UP / 0x6F9DE4DE {
            id_string32: u32,
            unsigned_value: int,
        }
    }

    rpc! {
        /// set_local_scale (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `arg2`: float, 8 bits over 0..1
        SetLocalScale / SET_LOCAL_SCALE / 0x6FA6C5EA {
            entity: int,
            id_string32: u32,
            arg2: float,
        }
    }

    rpc! {
        /// sync_galactic_war_joint_operation:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `unsigned_value_4`: int, 32 bits, 0..0
        SyncGalacticWarJointOperation / SYNC_GALACTIC_WAR_JOINT_OPERATION / 0x6FFDFC9F {
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
            unsigned_value_4: int,
        }
    }

    rpc! {
        /// hostability_scores (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 5 x u64
        /// - `arg1`: up to 5 x float, 31 bits over -10000..10000
        HostabilityScores / HOSTABILITY_SCORES / 0x709C1E3C {
            arg0: [u64; 5],
            arg1: [float; 5],
        }
    }

    rpc! {
        /// request_hostinfo:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: u64
        RequestHostinfo / REQUEST_HOSTINFO / 0x70D2CEEB {
            arg0: u64,
            arg1: u64,
        }
    }

    rpc! {
        /// set_enabled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetEnabled / SET_ENABLED / 0x70D730CE {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// hotjoin_sync_film_progress (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        HotjoinSyncFilmProgress / HOTJOIN_SYNC_FILM_PROGRESS / 0x70D8FA3C {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// set_hellpod_occupied (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetHellpodOccupied / SET_HELLPOD_OCCUPIED / 0x70DD8901 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// from_host_set_start_position:
        ///
        /// Arguments:
        /// - `vector3`: vector3, 31 bits over -10000..10000
        FromHostSetStartPosition / FROM_HOST_SET_START_POSITION / 0x7131EF5E {
            vector3: vector3,
        }
    }

    rpc! {
        /// from_host_set_ship_seed:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        FromHostSetShipSeed / FROM_HOST_SET_SHIP_SEED / 0x715A4644 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// from_host_set_generation_seed:
        ///
        /// Arguments:
        /// - `network_string`: string, at most 2047 bytes
        FromHostSetGenerationSeed / FROM_HOST_SET_GENERATION_SEED / 0x716C6176 {
            network_string: string,
        }
    }

    rpc! {
        /// stratagem_confirmation_result (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        StratagemConfirmationResult / STRATAGEM_CONFIRMATION_RESULT / 0x719E68DE {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// request_booster:
        ///
        /// Arguments:
        /// - `booster`: int, 5 bits, 0..31
        RequestBooster / REQUEST_BOOSTER / 0x71C061C5 {
            booster: int,
        }
    }

    rpc! {
        /// from_host_set_debug_level:
        ///
        /// Arguments:
        /// - `id_string64`: id64, a name hash
        FromHostSetDebugLevel / FROM_HOST_SET_DEBUG_LEVEL / 0x71C7CBFB {
            id_string64: id64,
        }
    }

    rpc! {
        /// abort_dialogue (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        AbortDialogue / ABORT_DIALOGUE / 0x71E946F0 {
            entity: int,
        }
    }

    rpc! {
        /// increment_total_number_of_dropped_loot (session-bound): no arguments.
        IncrementTotalNumberOfDroppedLoot / INCREMENT_TOTAL_NUMBER_OF_DROPPED_LOOT / 0x723D4A53;
    }

    rpc! {
        /// loot_pick_up_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        LootPickUpRequest / LOOT_PICK_UP_REQUEST / 0x72E20E4E {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// switch_script_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        SwitchScriptEvent / SWITCH_SCRIPT_EVENT / 0x72F8D865 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// sync_minefield (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `arg3`: up to 48 x u64
        /// - `bool_value`: bool
        SyncMinefield / SYNC_MINEFIELD / 0x7346BDF7 {
            entity: int,
            vector3: vector3,
            float_value: float,
            arg3: [u64; 48],
            bool_value: bool,
        }
    }

    rpc! {
        /// material_swap (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        MaterialSwap / MATERIAL_SWAP / 0x73B0FC17 {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// set_sight_node (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        SetSightNode / SET_SIGHT_NODE / 0x7406790F {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// enable_autoload (session-bound): no arguments.
        EnableAutoload / ENABLE_AUTOLOAD / 0x74093387;
    }

    rpc! {
        /// register_kill (session-bound):
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `damage_type`: int, 4 bits, 0..15
        /// - `element_type`: int, 4 bits, 0..15
        RegisterKill / REGISTER_KILL / 0x746B9240 {
            peer_id: u64,
            entity: int,
            entity_2: int,
            damage_type: int,
            element_type: int,
        }
    }

    rpc! {
        /// set_align_strategy (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `align_strategy`: int, 4 bits, 0..15
        SetAlignStrategy / SET_ALIGN_STRATEGY / 0x749C4EFF {
            entity: int,
            align_strategy: int,
        }
    }

    rpc! {
        /// set_minigame_value_alignment_corrected_value (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        SetMinigameValueAlignmentCorrectedValue / SET_MINIGAME_VALUE_ALIGNMENT_CORRECTED_VALUE / 0x754EAB63 {
            entity: int,
            unsigned_value: int,
            bool_value: bool,
            float_value: float,
        }
    }

    rpc! {
        /// reset_align_strategy (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ResetAlignStrategy / RESET_ALIGN_STRATEGY / 0x755D589E {
            entity: int,
        }
    }

    rpc! {
        /// terminal_interact_success (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        TerminalInteractSuccess / TERMINAL_INTERACT_SUCCESS / 0x7576EA66 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// delete_all_passengers_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DeleteAllPassengersRequest / DELETE_ALL_PASSENGERS_REQUEST / 0x76E54917 {
            entity: int,
        }
    }

    rpc! {
        /// trigger_attack_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        TriggerAttackEvent / TRIGGER_ATTACK_EVENT / 0x76F91B8D {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// on_stagger (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        /// - `entity_2`: int, 15 bits, 0..32767
        OnStagger / ON_STAGGER / 0x777AC34B {
            entity: int,
            vector3: vector3,
            unsigned_value: int,
            bool_value: bool,
            entity_2: int,
        }
    }

    rpc! {
        /// weapon_execute_chambering (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        WeaponExecuteChambering / WEAPON_EXECUTE_CHAMBERING / 0x778122AA {
            entity: int,
        }
    }

    rpc! {
        /// add_beam (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        AddBeam / ADD_BEAM / 0x77E03EEA {
            entity: int,
        }
    }

    rpc! {
        /// shuttle_extraction (session-bound):
        ///
        /// Arguments:
        /// - `bool_value`: bool
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `float_value`: float, 31 bits over -10000..10000
        ShuttleExtraction / SHUTTLE_EXTRACTION / 0x782D3B44 {
            bool_value: bool,
            entity: int,
            vector3: vector3,
            float_value: float,
        }
    }

    rpc! {
        /// finalize_swap_request:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: int, 15 bits, 0..32767
        FinalizeSwapRequest / FINALIZE_SWAP_REQUEST / 0x7864502F {
            arg0: u64,
            arg1: int,
        }
    }

    rpc! {
        /// mount_entities (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        MountEntities / MOUNT_ENTITIES / 0x78B68DB8 {
            entity: int,
        }
    }

    rpc! {
        /// arm_explosive (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ArmExplosive / ARM_EXPLOSIVE / 0x78D5D986 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// revive (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        Revive / REVIVE / 0x794AE2E0 {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// set_active (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetActive / SET_ACTIVE / 0x7A676B59 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// request_teleport_slot (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 15 bits, 0..32767
        RequestTeleportSlot / REQUEST_TELEPORT_SLOT / 0x7A70FCA4 {
            arg0: int,
        }
    }

    rpc! {
        /// play_hologram_event (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        PlayHologramEvent / PLAY_HOLOGRAM_EVENT / 0x7AC88323 {
            id_string32: u32,
            id_string32_2: u32,
        }
    }

    rpc! {
        /// abort_terminal_interact (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        AbortTerminalInteract / ABORT_TERMINAL_INTERACT / 0x7BA6E742 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// change_stack_value (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: int, 7 bits, 0..127
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        ChangeStackValue / CHANGE_STACK_VALUE / 0x7D488143 {
            entity: int,
            unsigned_value: int,
            arg2: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// from_host_finish_ship_spawn (session-bound): no arguments.
        FromHostFinishShipSpawn / FROM_HOST_FINISH_SHIP_SPAWN / 0x7DADD698;
    }

    rpc! {
        /// on_space_station_mismatch_detected (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        OnSpaceStationMismatchDetected / ON_SPACE_STATION_MISMATCH_DETECTED / 0x7E65303C {
            id_string32: u32,
        }
    }

    rpc! {
        /// select_drop_point (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        SelectDropPoint / SELECT_DROP_POINT / 0x7ED9E660 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// activate_cooldown (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        ActivateCooldown / ACTIVATE_COOLDOWN / 0x7EEB98A5 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// set_eye_type (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `eye_type`: int, 3 bits, 0..7
        SetEyeType / SET_EYE_TYPE / 0x7F29CB51 {
            entity: int,
            eye_type: int,
        }
    }

    rpc! {
        /// fancy_effect_system_recieve_effect_from_host (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `network_string`: string, at most 2047 bytes
        /// - `network_string_2`: string, at most 2047 bytes
        /// - `arg3`: up to 20 x int, 32 bits, 0..0
        FancyEffectSystemRecieveEffectFromHost / FANCY_EFFECT_SYSTEM_RECIEVE_EFFECT_FROM_HOST / 0x7F7C8316 {
            unsigned_value: int,
            network_string: string,
            network_string_2: string,
            arg3: [int; 20],
        }
    }

    rpc! {
        /// on_commend (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        OnCommend / ON_COMMEND / 0x7F919495 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// remote_trigger_event_on_entity (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        RemoteTriggerEventOnEntity / REMOTE_TRIGGER_EVENT_ON_ENTITY / 0x7F925EF7 {
            id_string32: u32,
            entity: int,
            bool_value: bool,
            float_value: float,
        }
    }

    rpc! {
        /// on_objective_stratagem_destroyed (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 8 bits, 0..255
        OnObjectiveStratagemDestroyed / ON_OBJECTIVE_STRATAGEM_DESTROYED / 0x7FCBC6A1 {
            arg0: int,
        }
    }

    rpc! {
        /// set_sticky_is_attached (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetStickyIsAttached / SET_STICKY_IS_ATTACHED / 0x8093823E {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// reinforce_score_increase (session-bound): no arguments.
        ReinforceScoreIncrease / REINFORCE_SCORE_INCREASE / 0x8123DD5A;
    }

    rpc! {
        /// player_invited_to_clan (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        PlayerInvitedToClan / PLAYER_INVITED_TO_CLAN / 0x81B6AD4C {
            entity: int,
        }
    }

    rpc! {
        /// mission_over_result (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 2 bits, 0..3
        MissionOverResult / MISSION_OVER_RESULT / 0x826B1386 {
            arg0: int,
        }
    }

    rpc! {
        /// set_production_cooldown_override (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        SetProductionCooldownOverride / SET_PRODUCTION_COOLDOWN_OVERRIDE / 0x82D4706D {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// from_migrated_host_state_requested: no arguments.
        FromMigratedHostStateRequested / FROM_MIGRATED_HOST_STATE_REQUESTED / 0x83C00A61;
    }

    rpc! {
        /// mission_end_info_request (session-bound): no arguments.
        MissionEndInfoRequest / MISSION_END_INFO_REQUEST / 0x83CCFD89;
    }

    rpc! {
        /// enable_and_show_shield_zone_actors (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        EnableAndShowShieldZoneActors / ENABLE_AND_SHOW_SHIELD_ZONE_ACTORS / 0x83F59D74 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// remove_all_proposed_drop_points (session-bound): no arguments.
        RemoveAllProposedDropPoints / REMOVE_ALL_PROPOSED_DROP_POINTS / 0x8407B78A;
    }

    rpc! {
        /// set_curve_navigation_enabled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetCurveNavigationEnabled / SET_CURVE_NAVIGATION_ENABLED / 0x84588375 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// from_client_verify_matching_latest_config:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        FromClientVerifyMatchingLatestConfig / FROM_CLIENT_VERIFY_MATCHING_LATEST_CONFIG / 0x847B2030 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// notify_dropoff_location (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 4 bits, 0..15
        NotifyDropoffLocation / NOTIFY_DROPOFF_LOCATION / 0x84B0404A {
            entity: int,
            entity_2: int,
            arg2: int,
        }
    }

    rpc! {
        /// arcade_stratagem_updated_highscores (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 10 x string, at most 2047 bytes
        /// - `arg1`: up to 10 x int, 32 bits, 0..0
        ArcadeStratagemUpdatedHighscores / ARCADE_STRATAGEM_UPDATED_HIGHSCORES / 0x8505580B {
            arg0: [string; 10],
            arg1: [int; 10],
        }
    }

    rpc! {
        /// removed_level_entities (session-bound):
        ///
        /// Arguments:
        /// - `removed_level_entities`: up to 128 x int, 32 bits, 0..0
        RemovedLevelEntities / REMOVED_LEVEL_ENTITIES / 0x8528235A {
            removed_level_entities: [int; 128],
        }
    }

    rpc! {
        /// sync_galactic_war_begin:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `int_value`: int, 32 bits, 0..0
        /// - `arg2`: u64
        /// - `id_string32`: u32
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        SyncGalacticWarBegin / SYNC_GALACTIC_WAR_BEGIN / 0x8532B643 {
            unsigned_value: int,
            int_value: int,
            arg2: u64,
            id_string32: u32,
            unsigned_value_2: int,
            unsigned_value_3: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// shared_weapon_state:
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `arg1`: up to 9 x int, 32 bits, 0..0
        /// - `arg2`: up to 9 x int, 32 bits, 0..0
        /// - `arg3`: int, 2 bits, 0..3
        SharedWeaponState / SHARED_WEAPON_STATE / 0x856EB26B {
            id_string32: u32,
            arg1: [int; 9],
            arg2: [int; 9],
            arg3: int,
        }
    }

    rpc! {
        /// avatar_change_locomotion_set (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        AvatarChangeLocomotionSet / AVATAR_CHANGE_LOCOMOTION_SET / 0x85D82942 {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// stimed (session-bound): no arguments.
        Stimed / STIMED / 0x85E22DD3;
    }

    rpc! {
        /// set_cloaked (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetCloaked / SET_CLOAKED / 0x861256C9 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// trigger_encounter (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 3 bits, 0..7
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        TriggerEncounter / TRIGGER_ENCOUNTER / 0x86133BB1 {
            arg0: int,
            vector3: vector3,
            entity: int,
            entity_2: int,
            vector3_2: vector3,
        }
    }

    rpc! {
        /// minigame_completed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        MinigameCompleted / MINIGAME_COMPLETED / 0x86A0897B {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// deduct_from_deposit (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 10 bits, 0..1023
        DeductFromDeposit / DEDUCT_FROM_DEPOSIT / 0x86A4E082 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// impact_play_effect (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        ImpactPlayEffect / IMPACT_PLAY_EFFECT / 0x874136EB {
            unsigned_value: int,
            vector3: vector3,
            vector3_2: vector3,
        }
    }

    rpc! {
        /// hellpad_spawn_stratagem (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 8 bits, 0..255
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        HellpadSpawnStratagem / HELLPAD_SPAWN_STRATAGEM / 0x87689FB6 {
            arg0: int,
            entity: int,
            entity_2: int,
            unsigned_value: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// set_debug_transform (session-bound):
        ///
        /// Arguments:
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        SetDebugTransform / SET_DEBUG_TRANSFORM / 0x8844FDFD {
            vector3: vector3,
            quaternion: quaternion,
        }
    }

    rpc! {
        /// set_ingame_ship_movement (session-bound):
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        SetIngameShipMovement / SET_INGAME_SHIP_MOVEMENT / 0x88EC1B6E {
            peer_id: u64,
            vector3: vector3,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// destructible_set_invincible (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        DestructibleSetInvincible / DESTRUCTIBLE_SET_INVINCIBLE / 0x8A793ACB {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// create_smoke_effect (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `vector3_3`: vector3, 31 bits over -10000..10000
        CreateSmokeEffect / CREATE_SMOKE_EFFECT / 0x8BA056B7 {
            entity: int,
            unsigned_value: int,
            vector3: vector3,
            vector3_2: vector3,
            vector3_3: vector3,
        }
    }

    rpc! {
        /// sync_galactic_war_time:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg1`: u64
        SyncGalacticWarTime / SYNC_GALACTIC_WAR_TIME / 0x8BBFF85F {
            unsigned_value: int,
            arg1: u64,
        }
    }

    rpc! {
        /// material_variable_vector2 (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        MaterialVariableVector2 / MATERIAL_VARIABLE_VECTOR2 / 0x8C77C638 {
            entity: int,
            arg1: int,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// player_abandoned (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        PlayerAbandoned / PLAYER_ABANDONED / 0x8E3CE42E {
            entity: int,
        }
    }

    rpc! {
        /// sync_galactic_war_persisted_operation:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `unsigned_value_4`: int, 32 bits, 0..0
        /// - `unsigned_value_5`: int, 32 bits, 0..0
        /// - `unsigned_value_6`: int, 32 bits, 0..0
        /// - `unsigned_value_7`: int, 32 bits, 0..0
        /// - `arg7`: int, 4 bits, 0..15
        /// - `unsigned_value_8`: int, 32 bits, 0..0
        /// - `arg9`: int, 3 bits, 0..7
        /// - `unsigned_value_9`: int, 32 bits, 0..0
        /// - `unsigned_value_10`: int, 32 bits, 0..0
        /// - `arg12`: up to 8 x int, 32 bits, 0..0
        SyncGalacticWarPersistedOperation / SYNC_GALACTIC_WAR_PERSISTED_OPERATION / 0x8F90532A {
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
            unsigned_value_4: int,
            unsigned_value_5: int,
            unsigned_value_6: int,
            unsigned_value_7: int,
            arg7: int,
            unsigned_value_8: int,
            arg9: int,
            unsigned_value_9: int,
            unsigned_value_10: int,
            arg12: [int; 8],
        }
    }

    rpc! {
        /// from_host_peer_added_to_session:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        FromHostPeerAddedToSession / FROM_HOST_PEER_ADDED_TO_SESSION / 0x8FE70196 {
            peer_id: u64,
        }
    }

    rpc! {
        /// sync_ragdoll_to_pose (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        SyncRagdollToPose / SYNC_RAGDOLL_TO_POSE / 0x907F14C5 {
            entity: int,
        }
    }

    rpc! {
        /// skip_arming_timer (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        SkipArmingTimer / SKIP_ARMING_TIMER / 0x90B2AD0D {
            entity: int,
        }
    }

    rpc! {
        /// set_visibility_group (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `bool_value`: bool
        SetVisibilityGroup / SET_VISIBILITY_GROUP / 0x9146C292 {
            entity: int,
            id_string32: u32,
            bool_value: bool,
        }
    }

    rpc! {
        /// stratagem_failed (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        StratagemFailed / STRATAGEM_FAILED / 0x91C0AD2B {
            unsigned_value: int,
        }
    }

    rpc! {
        /// load_and_play_extraction_cutscene (session-bound): no arguments.
        LoadAndPlayExtractionCutscene / LOAD_AND_PLAY_EXTRACTION_CUTSCENE / 0x920B8326;
    }

    rpc! {
        /// drop_entity (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DropEntity / DROP_ENTITY / 0x923D6DD8 {
            entity: int,
        }
    }

    rpc! {
        /// modifier_system_initialise_sync (session-bound): no arguments.
        ModifierSystemInitialiseSync / MODIFIER_SYSTEM_INITIALISE_SYNC / 0x932E9EC3;
    }

    rpc! {
        /// remove_proposed_drop_point (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        RemoveProposedDropPoint / REMOVE_PROPOSED_DROP_POINT / 0x93978882 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// dyndest_client_request_hide_and_spawn (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        /// - `arg3`: u64
        DyndestClientRequestHideAndSpawn / DYNDEST_CLIENT_REQUEST_HIDE_AND_SPAWN / 0x93E7ECA3 {
            entity: int,
            id_string32: u32,
            id_string32_2: u32,
            arg3: u64,
        }
    }

    rpc! {
        /// from_client_ready_for_synced_tasks:
        ///
        /// Arguments:
        /// - `bool_value`: bool
        FromClientReadyForSyncedTasks / FROM_CLIENT_READY_FOR_SYNCED_TASKS / 0x93ED0CCC {
            bool_value: bool,
        }
    }

    rpc! {
        /// update_hold_minigame_value (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        UpdateHoldMinigameValue / UPDATE_HOLD_MINIGAME_VALUE / 0x941CF43D {
            entity: int,
            unsigned_value: int,
            float_value: float,
        }
    }

    rpc! {
        /// set_entering (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 4 bits, 0..15
        SetEntering / SET_ENTERING / 0x943369BC {
            entity: int,
            entity_2: int,
            arg2: int,
        }
    }

    rpc! {
        /// attach_simple (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        AttachSimple / ATTACH_SIMPLE / 0x947D285A {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// score_sync_weapon_float (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `id_string64`: id64, a name hash
        /// - `arg2`: float, 31 bits over 0..10000000
        ScoreSyncWeaponFloat / SCORE_SYNC_WEAPON_FLOAT / 0x948A9FBC {
            id_string32: u32,
            id_string64: id64,
            arg2: float,
        }
    }

    rpc! {
        /// extraction_zone_discovered (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ExtractionZoneDiscovered / EXTRACTION_ZONE_DISCOVERED / 0x95B2503C {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// snap_rotation (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `quaternion`: quaternion
        SnapRotation / SNAP_ROTATION / 0x95D6B006 {
            entity: int,
            quaternion: quaternion,
        }
    }

    rpc! {
        /// trigger_particle_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 5 bits, 0..31
        TriggerParticleEvent / TRIGGER_PARTICLE_EVENT / 0x95D8E79E {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// set_ingame_ship_transform (session-bound):
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        SetIngameShipTransform / SET_INGAME_SHIP_TRANSFORM / 0x9625DF78 {
            peer_id: u64,
            vector3: vector3,
            vector3_2: vector3,
        }
    }

    rpc! {
        /// sync_shared_unit_customizations:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: up to 12 x int, 32 bits, 0..0
        SyncSharedUnitCustomizations / SYNC_SHARED_UNIT_CUSTOMIZATIONS / 0x9853E94C {
            peer_id: u64,
            arg1: [int; 12],
        }
    }

    rpc! {
        /// mission_started (session-bound):
        ///
        /// Arguments:
        /// - `network_string`: string, at most 2047 bytes
        MissionStarted / MISSION_STARTED / 0x98722290 {
            network_string: string,
        }
    }

    rpc! {
        /// set_ability_paused (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetAbilityPaused / SET_ABILITY_PAUSED / 0x99649625 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// pickup_payload (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 3 bits, 0..7
        PickupPayload / PICKUP_PAYLOAD / 0x99B0C1BB {
            entity: int,
            entity_2: int,
            arg2: int,
        }
    }

    rpc! {
        /// from_host_ready_peer_added (session-bound):
        ///
        /// Arguments:
        /// - `peer_id`: u64
        FromHostReadyPeerAdded / FROM_HOST_READY_PEER_ADDED / 0x9A4F9510 {
            peer_id: u64,
        }
    }

    rpc! {
        /// on_auto_reinforce (session-bound): no arguments.
        OnAutoReinforce / ON_AUTO_REINFORCE / 0x9B3DECA7;
    }

    rpc! {
        /// targeting_decal (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `id_string64`: id64, a name hash
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        TargetingDecal / TARGETING_DECAL / 0x9BCBB84D {
            entity: int,
            entity_2: int,
            id_string64: id64,
            vector3: vector3,
            unsigned_value: int,
        }
    }

    rpc! {
        /// fire_system_delta_cell_list (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 64 x int, 32 bits, 0..0
        FireSystemDeltaCellList / FIRE_SYSTEM_DELTA_CELL_LIST / 0x9BEAB4DC {
            arg0: [int; 64],
        }
    }

    rpc! {
        /// from_host_set_shipname_start: no arguments.
        FromHostSetShipnameStart / FROM_HOST_SET_SHIPNAME_START / 0x9BF86D8C;
    }

    rpc! {
        /// set_job_position (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        SetJobPosition / SET_JOB_POSITION / 0x9C93533D {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// delete_current_equipment (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DeleteCurrentEquipment / DELETE_CURRENT_EQUIPMENT / 0x9D1C39F9 {
            entity: int,
        }
    }

    rpc! {
        /// corpsed_reattach (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        CorpsedReattach / CORPSED_REATTACH / 0x9D405575 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// give_ammo (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        GiveAmmo / GIVE_AMMO / 0x9E7FA5BA {
            entity: int,
        }
    }

    rpc! {
        /// replicate_override_batch:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `arg3`: up to 20 x int, 32 bits, 0..0
        /// - `arg4`: up to 20 x int, 32 bits, 0..0
        /// - `arg5`: up to 20 x int, 32 bits, 0..0
        /// - `arg6`: up to 20 x u64
        ReplicateOverrideBatch / REPLICATE_OVERRIDE_BATCH / 0x9F1970AC {
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
            arg3: [int; 20],
            arg4: [int; 20],
            arg5: [int; 20],
            arg6: [u64; 20],
        }
    }

    rpc! {
        /// from_host_abort_network_play (session-bound): no arguments.
        FromHostAbortNetworkPlay / FROM_HOST_ABORT_NETWORK_PLAY / 0x9F2F310F;
    }

    rpc! {
        /// set_armor_modifier (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        SetArmorModifier / SET_ARMOR_MODIFIER / 0x9F9C90BF {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// ingame_chat_message:
        ///
        /// Arguments:
        /// - `arg0`: up to 128 x int, 32 bits, 0..0
        IngameChatMessage / INGAME_CHAT_MESSAGE / 0x9FDDB88E {
            arg0: [int; 128],
        }
    }

    rpc! {
        /// from_host_set_game_state:
        ///
        /// Arguments:
        /// - `arg0`: int, 3 bits, 0..7
        FromHostSetGameState / FROM_HOST_SET_GAME_STATE / 0xA04318B9 {
            arg0: int,
        }
    }

    rpc! {
        /// request_set_parent_active (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        RequestSetParentActive / REQUEST_SET_PARENT_ACTIVE / 0xA057304F {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// mission_activity_result (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 16 x int, 32 bits, 0..0
        /// - `arg1`: up to 16 x int, 32 bits, 0..0
        MissionActivityResult / MISSION_ACTIVITY_RESULT / 0xA0823945 {
            arg0: [int; 16],
            arg1: [int; 16],
        }
    }

    rpc! {
        /// loot_destroyed_message (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        LootDestroyedMessage / LOOT_DESTROYED_MESSAGE / 0xA1A42E24 {
            entity: int,
        }
    }

    rpc! {
        /// radio (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        Radio / RADIO / 0xA1B06B3A {
            id_string32: u32,
        }
    }

    rpc! {
        /// spawn_minefield_mines (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        /// - `unsigned_value`: int, 32 bits, 0..0
        SpawnMinefieldMines / SPAWN_MINEFIELD_MINES / 0xA205F4FC {
            entity: int,
            vector3: vector3,
            quaternion: quaternion,
            unsigned_value: int,
        }
    }

    rpc! {
        /// trigger_shipmaster_greeting (session-bound): no arguments.
        TriggerShipmasterGreeting / TRIGGER_SHIPMASTER_GREETING / 0xA2447D71;
    }

    rpc! {
        /// set_actor_motion_property (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        SetActorMotionProperty / SET_ACTOR_MOTION_PROPERTY / 0xA2668EEA {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// set_minigame_reels_selected_reel (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetMinigameReelsSelectedReel / SET_MINIGAME_REELS_SELECTED_REEL / 0xA2B1528B {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// set_interested_dialogue_target (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `peer_id`: u64
        SetInterestedDialogueTarget / SET_INTERESTED_DIALOGUE_TARGET / 0xA2DE29C6 {
            entity: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// trigger_intel_encounter (session-bound):
        ///
        /// Arguments:
        /// - `vector3`: vector3, 31 bits over -10000..10000
        TriggerIntelEncounter / TRIGGER_INTEL_ENCOUNTER / 0xA36A013E {
            vector3: vector3,
        }
    }

    rpc! {
        /// change_equipment (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        ChangeEquipment / CHANGE_EQUIPMENT / 0xA4617570 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// from_host_added_to_session (session-bound): no arguments.
        FromHostAddedToSession / FROM_HOST_ADDED_TO_SESSION / 0xA497A2DD;
    }

    rpc! {
        /// set_active_passenger (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetActivePassenger / SET_ACTIVE_PASSENGER / 0xA4A69BA2 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// from_client_completed_loadout_load:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        FromClientCompletedLoadoutLoad / FROM_CLIENT_COMPLETED_LOADOUT_LOAD / 0xA4E4ECFC {
            peer_id: u64,
            unsigned_value: int,
        }
    }

    rpc! {
        /// on_notified (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        OnNotified / ON_NOTIFIED / 0xA62E6EA9 {
            entity: int,
            unsigned_value: int,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// score_sync_weapon_bool (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `id_string64`: id64, a name hash
        /// - `unsigned_value`: int, 32 bits, 0..0
        ScoreSyncWeaponBool / SCORE_SYNC_WEAPON_BOOL / 0xA7BEF9AA {
            id_string32: u32,
            id_string64: id64,
            unsigned_value: int,
        }
    }

    rpc! {
        /// set_damage_owner (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        SetDamageOwner / SET_DAMAGE_OWNER / 0xA7CF9A72 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// request_cycle_seat (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `seat_index`: int, 5 bits, -1..30
        /// - `arg3`: int, 1 bits, 0..1
        RequestCycleSeat / REQUEST_CYCLE_SEAT / 0xA7ECE676 {
            entity: int,
            entity_2: int,
            seat_index: int,
            arg3: int,
        }
    }

    rpc! {
        /// external_switch_state_reset (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ExternalSwitchStateReset / EXTERNAL_SWITCH_STATE_RESET / 0xA8909C39 {
            entity: int,
        }
    }

    rpc! {
        /// set_switch_value (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: float, 16 bits over 0..1
        SetSwitchValue / SET_SWITCH_VALUE / 0xA8C69F94 {
            entity: int,
            arg1: float,
        }
    }

    rpc! {
        /// stratagem_ball_sound (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        StratagemBallSound / STRATAGEM_BALL_SOUND / 0xA8E4D158 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// drop_select_done (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DropSelectDone / DROP_SELECT_DONE / 0xA90979A8 {
            entity: int,
        }
    }

    rpc! {
        /// dialogue_response (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        DialogueResponse / DIALOGUE_RESPONSE / 0xA9B1F8BE {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// loot_pick_up (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        LootPickUp / LOOT_PICK_UP / 0xAA8B3413 {
            entity: int,
            entity_2: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// score_sync_float (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `arg1`: float, 31 bits over 0..10000000
        ScoreSyncFloat / SCORE_SYNC_FLOAT / 0xAA9AE405 {
            id_string32: u32,
            arg1: float,
        }
    }

    rpc! {
        /// shield_on_depleted (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ShieldOnDepleted / SHIELD_ON_DEPLETED / 0xAAB6C8B8 {
            entity: int,
        }
    }

    rpc! {
        /// roll_for_status_effect (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 7 bits, 0..127
        /// - `id_string32`: u32
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `entity_3`: int, 15 bits, 0..32767
        RollForStatusEffect / ROLL_FOR_STATUS_EFFECT / 0xAAC61EDF {
            entity: int,
            arg1: int,
            id_string32: u32,
            float_value: float,
            entity_2: int,
            entity_3: int,
        }
    }

    rpc! {
        /// recharge_set_timer (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        RechargeSetTimer / RECHARGE_SET_TIMER / 0xAAD558B0 {
            entity: int,
            float_value: float,
        }
    }

    rpc! {
        /// on_knockback (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `arg3`: float, 24 bits over 0..500000
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `entity_2`: int, 15 bits, 0..32767
        OnKnockback / ON_KNOCKBACK / 0xAC0939B0 {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            arg3: float,
            float_value: float,
            float_value_2: float,
            entity_2: int,
        }
    }

    rpc! {
        /// set_minigame_pipe_slide_offset_value (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        SetMinigamePipeSlideOffsetValue / SET_MINIGAME_PIPE_SLIDE_OFFSET_VALUE / 0xAC554F8C {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
        }
    }

    rpc! {
        /// set_inventory_visibility (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        SetInventoryVisibility / SET_INVENTORY_VISIBILITY / 0xAC608782 {
            entity: int,
            bool_value: bool,
            bool_value_2: bool,
        }
    }

    rpc! {
        /// synced_ability_hit (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        SyncedAbilityHit / SYNCED_ABILITY_HIT / 0xACC0A549 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// from_host_loadout_weapon_swap_ready (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: u64
        FromHostLoadoutWeaponSwapReady / FROM_HOST_LOADOUT_WEAPON_SWAP_READY / 0xACD14C03 {
            arg0: u64,
        }
    }

    rpc! {
        /// set_wants_to_burrow (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetWantsToBurrow / SET_WANTS_TO_BURROW / 0xACD9DF99 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// smooth_attach_offset_rotation (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        /// - `float_value`: float, 31 bits over -10000..10000
        SmoothAttachOffsetRotation / SMOOTH_ATTACH_OFFSET_ROTATION / 0xAEA87BDB {
            entity: int,
            entity_2: int,
            unsigned_value: int,
            vector3: vector3,
            quaternion: quaternion,
            float_value: float,
        }
    }

    rpc! {
        /// request_exit_seat_collection_denied (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        RequestExitSeatCollectionDenied / REQUEST_EXIT_SEAT_COLLECTION_DENIED / 0xAEE38814 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// set_animation_times (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 32 x float, 8 bits over 0..1
        SetAnimationTimes / SET_ANIMATION_TIMES / 0xAEFB3729 {
            entity: int,
            arg1: [float; 32],
        }
    }

    rpc! {
        /// trigger_hazard (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        TriggerHazard / TRIGGER_HAZARD / 0xAF4DD832 {
            entity: int,
        }
    }

    rpc! {
        /// fancy_effect_system_initialise_sync (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        FancyEffectSystemInitialiseSync / FANCY_EFFECT_SYSTEM_INITIALISE_SYNC / 0xAF636935 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// detonate_c4_charges (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DetonateC4Charges / DETONATE_C4_CHARGES / 0xAFA01C37 {
            entity: int,
        }
    }

    rpc! {
        /// audio_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        /// - `id_string32_3`: u32
        AudioEvent / AUDIO_EVENT / 0xB0E49571 {
            entity: int,
            id_string32: u32,
            id_string32_2: u32,
            id_string32_3: u32,
        }
    }

    rpc! {
        /// from_host_set_generation_hash:
        ///
        /// Arguments:
        /// - `arg0`: u64
        FromHostSetGenerationHash / FROM_HOST_SET_GENERATION_HASH / 0xB117F64F {
            arg0: u64,
        }
    }

    rpc! {
        /// voiceover_stop_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        /// - `float_value`: float, 31 bits over -10000..10000
        VoiceoverStopEvent / VOICEOVER_STOP_EVENT / 0xB166F3BC {
            entity: int,
            id_string32: u32,
            id_string32_2: u32,
            float_value: float,
        }
    }

    rpc! {
        /// shield_zone_depleted (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        ShieldZoneDepleted / SHIELD_ZONE_DEPLETED / 0xB2033929 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// mine_triggered (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        MineTriggered / MINE_TRIGGERED / 0xB29CD3AE {
            entity: int,
            unsigned_value: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// sync_stratagems:
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 16 x int, 32 bits, 0..0
        /// - `arg2`: up to 16 x int, 32 bits, 0..0
        /// - `arg3`: up to 16 x u64
        /// - `arg4`: up to 32 x int, 32 bits, 0..0
        /// - `arg5`: up to 16 x bool
        /// - `arg6`: u64
        SyncStratagems / SYNC_STRATAGEMS / 0xB2E77CAE {
            entity: int,
            arg1: [int; 16],
            arg2: [int; 16],
            arg3: [u64; 16],
            arg4: [int; 32],
            arg5: [bool; 16],
            arg6: u64,
        }
    }

    rpc! {
        /// sync_stratagem_loadout_to_hotjoiner: no arguments.
        SyncStratagemLoadoutToHotjoiner / SYNC_STRATAGEM_LOADOUT_TO_HOTJOINER / 0xB317CB48;
    }

    rpc! {
        /// debug_load_soundbank (session-bound):
        ///
        /// Arguments:
        /// - `id_string64`: id64, a name hash
        DebugLoadSoundbank / DEBUG_LOAD_SOUNDBANK / 0xB3890778 {
            id_string64: id64,
        }
    }

    rpc! {
        /// oil_rig_drilling_start (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        OilRigDrillingStart / OIL_RIG_DRILLING_START / 0xB38F718C {
            entity: int,
        }
    }

    rpc! {
        /// set_ragdoll_enabled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetRagdollEnabled / SET_RAGDOLL_ENABLED / 0xB390C486 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// start_upload_discovery (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        StartUploadDiscovery / START_UPLOAD_DISCOVERY / 0xB3C816CD {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// set_animation_states_large (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 32 x int, 32 bits, 0..0
        SetAnimationStatesLarge / SET_ANIMATION_STATES_LARGE / 0xB419B858 {
            entity: int,
            arg1: [int; 32],
        }
    }

    rpc! {
        /// give_stims (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `id_string64`: id64, a name hash
        GiveStims / GIVE_STIMS / 0xB42289F4 {
            entity: int,
            unsigned_value: int,
            id_string64: id64,
        }
    }

    rpc! {
        /// objective_link_to_guard_force_member (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ObjectiveLinkToGuardForceMember / OBJECTIVE_LINK_TO_GUARD_FORCE_MEMBER / 0xB45D681B {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// load_player_loadout_for_peer:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: int, 2 bits, 0..3
        /// - `arg2`: int, 1 bits, 0..1
        /// - `unsigned_value`: int, 32 bits, 0..0
        LoadPlayerLoadoutForPeer / LOAD_PLAYER_LOADOUT_FOR_PEER / 0xB4ACFE77 {
            peer_id: u64,
            arg1: int,
            arg2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// respond_to_emote (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        RespondToEmote / RESPOND_TO_EMOTE / 0xB4DC1B0C {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// cycle_seat_denied (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        CycleSeatDenied / CYCLE_SEAT_DENIED / 0xB6487191 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// charge_minigame_state_change (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        ChargeMinigameStateChange / CHARGE_MINIGAME_STATE_CHANGE / 0xB6623100 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// hotjoin_ability_synced_data (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        HotjoinAbilitySyncedData / HOTJOIN_ABILITY_SYNCED_DATA / 0xB6D6458F {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// build_wave_info (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        BuildWaveInfo / BUILD_WAVE_INFO / 0xB72EA1FF {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// sync_galactic_war_space_station:
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: up to 8 x int, 32 bits, 0..0
        SyncGalacticWarSpaceStation / SYNC_GALACTIC_WAR_SPACE_STATION / 0xB81B3BA0 {
            id_string32: u32,
            unsigned_value: int,
            arg2: [int; 8],
        }
    }

    rpc! {
        /// to_migration_host_event_list (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 64 x u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        ToMigrationHostEventList / TO_MIGRATION_HOST_EVENT_LIST / 0xB82A6556 {
            arg0: [u64; 64],
            unsigned_value: int,
        }
    }

    rpc! {
        /// add_projectile_to_manager (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `arg3`: int, 9 bits, 0..511
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `entity_3`: int, 15 bits, 0..32767
        /// - `peer_id`: u64
        AddProjectileToManager / ADD_PROJECTILE_TO_MANAGER / 0xB845EB3B {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            arg3: int,
            entity_2: int,
            entity_3: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// unmount_entity (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        UnmountEntity / UNMOUNT_ENTITY / 0xB852FD9D {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// score_sync_done (session-bound): no arguments.
        ScoreSyncDone / SCORE_SYNC_DONE / 0xB966BEE7;
    }

    rpc! {
        /// animation_event_with_params (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `animation_event`: int, 13 bits, 0..8191
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        AnimationEventWithParams / ANIMATION_EVENT_WITH_PARAMS / 0xB9A1407A {
            entity: int,
            animation_event: int,
            bool_value: bool,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// from_host_notify_new_host (session-bound):
        ///
        /// Arguments:
        /// - `peer_id`: u64
        FromHostNotifyNewHost / FROM_HOST_NOTIFY_NEW_HOST / 0xB9E77C36 {
            peer_id: u64,
        }
    }

    rpc! {
        /// equipment_charge_up (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        EquipmentChargeUp / EQUIPMENT_CHARGE_UP / 0xBA6C9490 {
            entity: int,
        }
    }

    rpc! {
        /// sync_mission_history:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SyncMissionHistory / SYNC_MISSION_HISTORY / 0xBB17CE45 {
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// modifier_system_finalise_sync (session-bound): no arguments.
        ModifierSystemFinaliseSync / MODIFIER_SYSTEM_FINALISE_SYNC / 0xBBC33FB9;
    }

    rpc! {
        /// from_host_set_shipnames:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        FromHostSetShipnames / FROM_HOST_SET_SHIPNAMES / 0xBBE961C3 {
            peer_id: u64,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// add_wound (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `int_value`: int, 32 bits, 0..0
        /// - `arg2`: vector3, 14 bits over -64..64
        /// - `arg3`: float, 8 bits over 0..8
        /// - `arg4`: int, 2 bits, 0..3
        AddWound / ADD_WOUND / 0xBC1BC9DD {
            entity: int,
            int_value: int,
            arg2: vector3,
            arg3: float,
            arg4: int,
        }
    }

    rpc! {
        /// sync_galactic_war_data_done: no arguments.
        SyncGalacticWarDataDone / SYNC_GALACTIC_WAR_DATA_DONE / 0xBCE2A8A2;
    }

    rpc! {
        /// verify_synced: no arguments.
        VerifySynced / VERIFY_SYNCED / 0xBD5E4AD9;
    }

    rpc! {
        /// from_partymember_join_finished:
        ///
        /// Arguments:
        /// - `network_string`: string, at most 2047 bytes
        FromPartymemberJoinFinished / FROM_PARTYMEMBER_JOIN_FINISHED / 0xBD5FAA86 {
            network_string: string,
        }
    }

    rpc! {
        /// convoy_reached_goal (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ConvoyReachedGoal / CONVOY_REACHED_GOAL / 0xBD92A352 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// abort_value_alignment_minigame (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        AbortValueAlignmentMinigame / ABORT_VALUE_ALIGNMENT_MINIGAME / 0xBDA3FCD4 {
            entity: int,
            unsigned_value: int,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// animation_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `animation_event`: int, 13 bits, 0..8191
        /// - `bool_value`: bool
        AnimationEvent / ANIMATION_EVENT / 0xBDE53653 {
            entity: int,
            animation_event: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// entity_commanded (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        EntityCommanded / ENTITY_COMMANDED / 0xBE23D120 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// set_position (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        SetPosition / SET_POSITION / 0xBE768C28 {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// dialogue_failed_none_found (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DialogueFailedNoneFound / DIALOGUE_FAILED_NONE_FOUND / 0xBF84D058 {
            entity: int,
        }
    }

    rpc! {
        /// on_destroyed_dynamic (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg4`: int, 8 bits, 0..255
        /// - `arg5`: int, 3 bits, 0..7
        /// - `arg6`: int, 1 bits, 0..1
        /// - `peer_id`: u64
        OnDestroyedDynamic / ON_DESTROYED_DYNAMIC / 0xBFA208DE {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            unsigned_value: int,
            arg4: int,
            arg5: int,
            arg6: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// mission_end_info_result (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 8 x int, 32 bits, 0..0
        MissionEndInfoResult / MISSION_END_INFO_RESULT / 0xC053D7F7 {
            arg0: [int; 8],
        }
    }

    rpc! {
        /// fire_source_trigger (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        FireSourceTrigger / FIRE_SOURCE_TRIGGER / 0xC1384643 {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// from_client_synced_task_done:
        ///
        /// Arguments:
        /// - `synced_task`: int, 2 bits, 0..3
        /// - `arg1`: int, 3 bits, 0..7
        FromClientSyncedTaskDone / FROM_CLIENT_SYNCED_TASK_DONE / 0xC237BE1C {
            synced_task: int,
            arg1: int,
        }
    }

    rpc! {
        /// change_status_effect_mask (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: up to 2 x u64
        ChangeStatusEffectMask / CHANGE_STATUS_EFFECT_MASK / 0xC2537DEB {
            entity: int,
            unsigned_value: int,
            arg2: [u64; 2],
        }
    }

    rpc! {
        /// panic_exit (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        PanicExit / PANIC_EXIT / 0xC29846F8 {
            entity: int,
        }
    }

    rpc! {
        /// set_throwable_path (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string64`: id64, a name hash
        SetThrowablePath / SET_THROWABLE_PATH / 0xC41012B5 {
            entity: int,
            id_string64: id64,
        }
    }

    rpc! {
        /// transport_dropped (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        TransportDropped / TRANSPORT_DROPPED / 0xC521A7FB {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// trigger_projectile_explosion (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `position`: vector3, 17 bits over -2000..2000
        /// - `direction`: direction
        /// - `arg3`: vector3, 14 bits over -2000..2000
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        /// - `arg6`: int, 9 bits, 0..511
        TriggerProjectileExplosion / TRIGGER_PROJECTILE_EXPLOSION / 0xC64D92F0 {
            unsigned_value: int,
            position: vector3,
            direction: direction,
            arg3: vector3,
            bool_value: bool,
            bool_value_2: bool,
            arg6: int,
        }
    }

    rpc! {
        /// force_exit_seat_collection (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `seat_index`: int, 5 bits, -1..30
        ForceExitSeatCollection / FORCE_EXIT_SEAT_COLLECTION / 0xC698216F {
            entity: int,
            seat_index: int,
        }
    }

    rpc! {
        /// set_filming_completed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        SetFilmingCompleted / SET_FILMING_COMPLETED / 0xC69906DA {
            entity: int,
        }
    }

    rpc! {
        /// ragdoll_set_velocity (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        RagdollSetVelocity / RAGDOLL_SET_VELOCITY / 0xC73AB797 {
            entity: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// drop_current_equipment (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 2 bits, 0..3
        DropCurrentEquipment / DROP_CURRENT_EQUIPMENT / 0xC73B62CA {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// request_state_change (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 2 bits, 0..3
        /// - `damage_type`: int, 4 bits, 0..15
        /// - `element_type`: int, 4 bits, 0..15
        RequestStateChange / REQUEST_STATE_CHANGE / 0xC877F4E4 {
            entity: int,
            arg1: int,
            damage_type: int,
            element_type: int,
        }
    }

    rpc! {
        /// held_throwable_cancelled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        HeldThrowableCancelled / HELD_THROWABLE_CANCELLED / 0xC88DD57B {
            entity: int,
        }
    }

    rpc! {
        /// play_looping_surface_effect (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 2 bits, 0..3
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `direction`: direction
        /// - `float_value`: float, 31 bits over -10000..10000
        PlayLoopingSurfaceEffect / PLAY_LOOPING_SURFACE_EFFECT / 0xC8F0EBB0 {
            entity: int,
            arg1: int,
            unsigned_value: int,
            direction: direction,
            float_value: float,
        }
    }

    rpc! {
        /// convoy_destroyed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ConvoyDestroyed / CONVOY_DESTROYED / 0xC92B0832 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// sync_player_history:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: u64
        /// - `arg2`: u64
        /// - `arg3`: u64
        /// - `arg4`: u64
        /// - `arg5`: u64
        /// - `bool_value`: bool
        /// - `bool_value_2`: bool
        /// - `bool_value_3`: bool
        /// - `bool_value_4`: bool
        /// - `bool_value_5`: bool
        /// - `bool_value_6`: bool
        /// - `arg12`: up to 16 x u32
        /// - `arg13`: up to 48 x int, 32 bits, 0..0
        /// - `arg14`: up to 48 x int, 32 bits, 0..0
        /// - `arg15`: up to 48 x int, 32 bits, 0..0
        /// - `arg16`: int, 2 bits, 0..3
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `booster`: int, 5 bits, 0..31
        /// - `unsigned_value_4`: int, 32 bits, 0..0
        /// - `unsigned_value_5`: int, 32 bits, 0..0
        /// - `arg23`: int, 10 bits, 0..1023
        /// - `arg24`: int, 10 bits, 0..1023
        /// - `arg25`: up to 16 x int, 32 bits, 0..0
        /// - `arg26`: up to 16 x int, 32 bits, 0..0
        /// - `arg27`: up to 16 x u64
        /// - `arg28`: up to 32 x int, 32 bits, 0..0
        /// - `arg29`: up to 16 x bool
        /// - `arg30`: up to 12 x int, 32 bits, 0..0
        /// - `unsigned_value_6`: int, 32 bits, 0..0
        SyncPlayerHistory / SYNC_PLAYER_HISTORY / 0xC953DB6D {
            peer_id: u64,
            arg1: u64,
            arg2: u64,
            arg3: u64,
            arg4: u64,
            arg5: u64,
            bool_value: bool,
            bool_value_2: bool,
            bool_value_3: bool,
            bool_value_4: bool,
            bool_value_5: bool,
            bool_value_6: bool,
            arg12: [u32; 16],
            arg13: [int; 48],
            arg14: [int; 48],
            arg15: [int; 48],
            arg16: int,
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
            booster: int,
            unsigned_value_4: int,
            unsigned_value_5: int,
            arg23: int,
            arg24: int,
            arg25: [int; 16],
            arg26: [int; 16],
            arg27: [u64; 16],
            arg28: [int; 32],
            arg29: [bool; 16],
            arg30: [int; 12],
            unsigned_value_6: int,
        }
    }

    rpc! {
        /// dyndest_hide_and_spawn (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        /// - `arg3`: int, 11 bits, 0..2047
        /// - `arg4`: u64
        DyndestHideAndSpawn / DYNDEST_HIDE_AND_SPAWN / 0xCA166B80 {
            entity: int,
            id_string32: u32,
            id_string32_2: u32,
            arg3: int,
            arg4: u64,
        }
    }

    rpc! {
        /// hotjoin_ability_time_modifiers (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: float, 10 bits over 0..20
        /// - `bool_value`: bool
        HotjoinAbilityTimeModifiers / HOTJOIN_ABILITY_TIME_MODIFIERS / 0xCAFE0F09 {
            entity: int,
            arg1: float,
            bool_value: bool,
        }
    }

    rpc! {
        /// run_ability_frame (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 12 bits, 0..4095
        /// - `unsigned_value`: int, 32 bits, 0..0
        RunAbilityFrame / RUN_ABILITY_FRAME / 0xCB37E756 {
            entity: int,
            arg1: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// player_feed_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `damage_type`: int, 4 bits, 0..15
        /// - `element_type`: int, 4 bits, 0..15
        /// - `peer_id`: u64
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `entity_3`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        PlayerFeedEvent / PLAYER_FEED_EVENT / 0xCB8D4784 {
            entity: int,
            bool_value: bool,
            damage_type: int,
            element_type: int,
            peer_id: u64,
            entity_2: int,
            entity_3: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// abort_minigame_interact (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        AbortMinigameInteract / ABORT_MINIGAME_INTERACT / 0xCBCD4370 {
            entity: int,
        }
    }

    rpc! {
        /// from_host_set_transition_target:
        ///
        /// Arguments:
        /// - `arg0`: int, 3 bits, 0..7
        FromHostSetTransitionTarget / FROM_HOST_SET_TRANSITION_TARGET / 0xCBCEEC09 {
            arg0: int,
        }
    }

    rpc! {
        /// set_external_switches_activate (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        SetExternalSwitchesActivate / SET_EXTERNAL_SWITCHES_ACTIVATE / 0xCBF9E8C1 {
            entity: int,
        }
    }

    rpc! {
        /// increment_stratagem_placed_count (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 8 bits, 0..255
        IncrementStratagemPlacedCount / INCREMENT_STRATAGEM_PLACED_COUNT / 0xCC3DA5DF {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// from_host_execute_loadout_weapon_swap:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `id_string32`: u32
        /// - `arg4`: up to 48 x int, 32 bits, 0..0
        /// - `arg5`: up to 48 x int, 32 bits, 0..0
        FromHostExecuteLoadoutWeaponSwap / FROM_HOST_EXECUTE_LOADOUT_WEAPON_SWAP / 0xCCC9284C {
            peer_id: u64,
            arg1: u64,
            unsigned_value: int,
            id_string32: u32,
            arg4: [int; 48],
            arg5: [int; 48],
        }
    }

    rpc! {
        /// animation_event_duration (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `animation_event`: int, 13 bits, 0..8191
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        AnimationEventDuration / ANIMATION_EVENT_DURATION / 0xCD14B0C6 {
            entity: int,
            animation_event: int,
            float_value: float,
            bool_value: bool,
        }
    }

    rpc! {
        /// arcade_stratagem_sound (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        ArcadeStratagemSound / ARCADE_STRATAGEM_SOUND / 0xCD38E995 {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// tes_77352_missing_ship_inventory: no arguments.
        Tes77352MissingShipInventory / TES_77352_MISSING_SHIP_INVENTORY / 0xCDD8DC4E;
    }

    rpc! {
        /// set_interpolation_target (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `vector3_2`: vector3, 31 bits over -10000..10000
        /// - `float_value`: float, 31 bits over -10000..10000
        SetInterpolationTarget / SET_INTERPOLATION_TARGET / 0xCDE86C46 {
            entity: int,
            vector3: vector3,
            vector3_2: vector3,
            float_value: float,
        }
    }

    rpc! {
        /// loot_container_pick_up (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: up to 8 x int, 32 bits, 0..0
        /// - `arg3`: up to 8 x int, 32 bits, 0..0
        LootContainerPickUp / LOOT_CONTAINER_PICK_UP / 0xCE6B1CF3 {
            entity: int,
            entity_2: int,
            arg2: [int; 8],
            arg3: [int; 8],
        }
    }

    rpc! {
        /// add_teammate_marker (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 5 bits, 0..31
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value_2`: bool
        /// - `bool_value_3`: bool
        AddTeammateMarker / ADD_TEAMMATE_MARKER / 0xCF01B01F {
            arg0: int,
            vector3: vector3,
            entity: int,
            bool_value: bool,
            entity_2: int,
            unsigned_value: int,
            bool_value_2: bool,
            bool_value_3: bool,
        }
    }

    rpc! {
        /// animation_event_speed_percent (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `animation_event`: int, 13 bits, 0..8191
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        AnimationEventSpeedPercent / ANIMATION_EVENT_SPEED_PERCENT / 0xCF6B6FFF {
            entity: int,
            animation_event: int,
            float_value: float,
            float_value_2: float,
            bool_value: bool,
        }
    }

    rpc! {
        /// from_host_level_generation_started (session-bound):
        ///
        /// Arguments:
        /// - `network_string`: string, at most 2047 bytes
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: int, 3 bits, 0..7
        /// - `network_string_2`: string, at most 2047 bytes
        /// - `network_string_3`: string, at most 2047 bytes
        FromHostLevelGenerationStarted / FROM_HOST_LEVEL_GENERATION_STARTED / 0xD07C222E {
            network_string: string,
            unsigned_value: int,
            arg2: int,
            network_string_2: string,
            network_string_3: string,
        }
    }

    rpc! {
        /// synced_health_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `int_value`: int, 32 bits, 0..0
        SyncedHealthRequest / SYNCED_HEALTH_REQUEST / 0xD07EB2B2 {
            entity: int,
            int_value: int,
        }
    }

    rpc! {
        /// attach_offset (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        AttachOffset / ATTACH_OFFSET / 0xD183CB6C {
            entity: int,
            entity_2: int,
            unsigned_value: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// animation_event_broadcast (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `animation_event`: int, 13 bits, 0..8191
        /// - `bool_value`: bool
        AnimationEventBroadcast / ANIMATION_EVENT_BROADCAST / 0xD1FB45A4 {
            entity: int,
            animation_event: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// remote_trigger_event_on_speaker (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        RemoteTriggerEventOnSpeaker / REMOTE_TRIGGER_EVENT_ON_SPEAKER / 0xD26CCFEB {
            id_string32: u32,
            id_string32_2: u32,
            bool_value: bool,
            float_value: float,
        }
    }

    rpc! {
        /// displacement_activated (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DisplacementActivated / DISPLACEMENT_ACTIVATED / 0xD37CE426 {
            entity: int,
        }
    }

    rpc! {
        /// fire_single_round (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `arg2`: int, 10 bits, 0..1023
        FireSingleRound / FIRE_SINGLE_ROUND / 0xD3CFA6E4 {
            entity: int,
            vector3: vector3,
            arg2: int,
        }
    }

    rpc! {
        /// sync_update_or_add_lobby:
        ///
        /// Arguments:
        /// - `int_value`: int, 32 bits, 0..0
        /// - `network_string`: string, at most 2047 bytes
        /// - `int_value_2`: int, 32 bits, 0..0
        /// - `int_value_3`: int, 32 bits, 0..0
        /// - `int_value_4`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        /// - `int_value_5`: int, 32 bits, 0..0
        /// - `int_value_6`: int, 32 bits, 0..0
        /// - `network_string_2`: string, at most 2047 bytes
        /// - `arg9`: int, 3 bits, 0..7
        /// - `arg10`: u64
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg12`: up to 4 x string, at most 2047 bytes
        /// - `arg13`: up to 4 x int, 32 bits, 0..0
        /// - `arg14`: up to 4 x int, 32 bits, 0..0
        /// - `arg15`: up to 4 x u64
        /// - `arg16`: up to 4 x int, 32 bits, 0..0
        /// - `arg17`: up to 4 x bool
        /// - `arg18`: up to 4 x bool
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `arg21`: up to 8 x int, 32 bits, 0..0
        /// - `arg22`: up to 4 x int, 32 bits, 0..0
        SyncUpdateOrAddLobby / SYNC_UPDATE_OR_ADD_LOBBY / 0xD3DC3D9B {
            int_value: int,
            network_string: string,
            int_value_2: int,
            int_value_3: int,
            int_value_4: int,
            bool_value: bool,
            int_value_5: int,
            int_value_6: int,
            network_string_2: string,
            arg9: int,
            arg10: u64,
            unsigned_value: int,
            arg12: [string; 4],
            arg13: [int; 4],
            arg14: [int; 4],
            arg15: [u64; 4],
            arg16: [int; 4],
            arg17: [bool; 4],
            arg18: [bool; 4],
            unsigned_value_2: int,
            unsigned_value_3: int,
            arg21: [int; 8],
            arg22: [int; 4],
        }
    }

    rpc! {
        /// migrate_objective_carry (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        MigrateObjectiveCarry / MIGRATE_OBJECTIVE_CARRY / 0xD4188F9D {
            entity: int,
        }
    }

    rpc! {
        /// trigger_avatar_comms_notify (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        TriggerAvatarCommsNotify / TRIGGER_AVATAR_COMMS_NOTIFY / 0xD48CEDB2 {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// from_partymember_ready_for_confirmation:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `peer_id`: u64
        /// - `arg2`: int, 3 bits, 0..7
        FromPartymemberReadyForConfirmation / FROM_PARTYMEMBER_READY_FOR_CONFIRMATION / 0xD49B7B39 {
            arg0: u64,
            peer_id: u64,
            arg2: int,
        }
    }

    rpc! {
        /// set_minigame_value_alignment_input (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        SetMinigameValueAlignmentInput / SET_MINIGAME_VALUE_ALIGNMENT_INPUT / 0xD4E88E4C {
            entity: int,
            unsigned_value: int,
            bool_value: bool,
            float_value: float,
        }
    }

    rpc! {
        /// seater_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `seat_index`: int, 5 bits, -1..30
        /// - `bool_value`: bool
        SeaterState / SEATER_STATE / 0xD4F97316 {
            entity: int,
            entity_2: int,
            seat_index: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// set_job (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `ai_job`: int, 4 bits, 0..15
        SetJob / SET_JOB / 0xD51C1356 {
            entity: int,
            ai_job: int,
        }
    }

    rpc! {
        /// set_physics_actor_enable (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `bool_value`: bool
        SetPhysicsActorEnable / SET_PHYSICS_ACTOR_ENABLE / 0xD58E1FF3 {
            entity: int,
            id_string32: u32,
            bool_value: bool,
        }
    }

    rpc! {
        /// give_equipment (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        GiveEquipment / GIVE_EQUIPMENT / 0xD5B79752 {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// sync_droppable_infinite_map_items (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 128 x int, 15 bits, 0..32767
        /// - `arg1`: up to 4 x u64
        SyncDroppableInfiniteMapItems / SYNC_DROPPABLE_INFINITE_MAP_ITEMS / 0xD5BB2687 {
            arg0: [int; 128],
            arg1: [u64; 4],
        }
    }

    rpc! {
        /// remove_encounters_by_controller (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        RemoveEncountersByController / REMOVE_ENCOUNTERS_BY_CONTROLLER / 0xD6A0AC07 {
            entity: int,
        }
    }

    rpc! {
        /// from_client_left:
        ///
        /// Arguments:
        /// - `leave_reason`: int, 2 bits, 0..3
        FromClientLeft / FROM_CLIENT_LEFT / 0xD706BC6F {
            leave_reason: int,
        }
    }

    rpc! {
        /// sync_assignment_kill (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg2`: u64
        SyncAssignmentKill / SYNC_ASSIGNMENT_KILL / 0xD7C28C54 {
            id_string32: u32,
            unsigned_value: int,
            arg2: u64,
        }
    }

    rpc! {
        /// material_variable_scalar (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        /// - `float_value`: float, 31 bits over -10000..10000
        MaterialVariableScalar / MATERIAL_VARIABLE_SCALAR / 0xD8685C5B {
            entity: int,
            arg1: int,
            float_value: float,
        }
    }

    rpc! {
        /// sync_mission_join_time:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: u64
        SyncMissionJoinTime / SYNC_MISSION_JOIN_TIME / 0xD8923100 {
            peer_id: u64,
            arg1: u64,
        }
    }

    rpc! {
        /// remove_member (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        RemoveMember / REMOVE_MEMBER / 0xD91666F2 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// to_migration_host_count (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        ToMigrationHostCount / TO_MIGRATION_HOST_COUNT / 0xD9AF8C68 {
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// from_client_requested_state:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `synced_task`: int, 2 bits, 0..3
        FromClientRequestedState / FROM_CLIENT_REQUESTED_STATE / 0xDA444685 {
            arg0: u64,
            synced_task: int,
        }
    }

    rpc! {
        /// set_tag (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `tag`: u64
        SetTag / SET_TAG / 0xDA460628 {
            entity: int,
            tag: u64,
        }
    }

    rpc! {
        /// from_host_level_generation_completed (session-bound): no arguments.
        FromHostLevelGenerationCompleted / FROM_HOST_LEVEL_GENERATION_COMPLETED / 0xDA639DBC;
    }

    rpc! {
        /// sync_galactic_war_done:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        SyncGalacticWarDone / SYNC_GALACTIC_WAR_DONE / 0xDACED591 {
            unsigned_value: int,
        }
    }

    rpc! {
        /// sync_player_loaded_loadout:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: up to 16 x u32
        /// - `arg2`: up to 48 x int, 32 bits, 0..0
        /// - `arg3`: up to 48 x int, 32 bits, 0..0
        /// - `arg4`: up to 48 x int, 32 bits, 0..0
        SyncPlayerLoadedLoadout / SYNC_PLAYER_LOADED_LOADOUT / 0xDAE80239 {
            peer_id: u64,
            arg1: [u32; 16],
            arg2: [int; 48],
            arg3: [int; 48],
            arg4: [int; 48],
        }
    }

    rpc! {
        /// minigame_revert (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        MinigameRevert / MINIGAME_REVERT / 0xDB212C0E {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// on_exploration_reward_request_start (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        OnExplorationRewardRequestStart / ON_EXPLORATION_REWARD_REQUEST_START / 0xDC56E506 {
            entity: int,
        }
    }

    rpc! {
        /// hold_minigame_state_change (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        HoldMinigameStateChange / HOLD_MINIGAME_STATE_CHANGE / 0xDC6AC00A {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// seater_set_action (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `seat_index`: int, 5 bits, -1..30
        /// - `seat_index_2`: int, 5 bits, -1..30
        /// - `arg3`: int, 7 bits, -1..126
        /// - `arg4`: float, 7 bits over 0..8
        SeaterSetAction / SEATER_SET_ACTION / 0xDCC32107 {
            entity: int,
            seat_index: int,
            seat_index_2: int,
            arg3: int,
            arg4: float,
        }
    }

    rpc! {
        /// sync_loadout:
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `id_string32_2`: u32
        /// - `id_string32_3`: u32
        /// - `arg3`: int, 2 bits, 0..3
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `booster`: int, 5 bits, 0..31
        /// - `unsigned_value_4`: int, 32 bits, 0..0
        /// - `unsigned_value_5`: int, 32 bits, 0..0
        /// - `unsigned_value_6`: int, 32 bits, 0..0
        /// - `unsigned_value_7`: int, 32 bits, 0..0
        /// - `arg12`: up to 16 x int, 32 bits, 0..0
        /// - `arg13`: up to 16 x int, 32 bits, 0..0
        /// - `arg14`: up to 12 x int, 32 bits, 0..0
        /// - `unsigned_value_8`: int, 32 bits, 0..0
        SyncLoadout / SYNC_LOADOUT / 0xDCCD3363 {
            id_string32: u32,
            id_string32_2: u32,
            id_string32_3: u32,
            arg3: int,
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
            booster: int,
            unsigned_value_4: int,
            unsigned_value_5: int,
            unsigned_value_6: int,
            unsigned_value_7: int,
            arg12: [int; 16],
            arg13: [int; 16],
            arg14: [int; 12],
            unsigned_value_8: int,
        }
    }

    rpc! {
        /// sync_customization_seed:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        SyncCustomizationSeed / SYNC_CUSTOMIZATION_SEED / 0xDD10592D {
            unsigned_value: int,
        }
    }

    rpc! {
        /// map_location_discovered (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        MapLocationDiscovered / MAP_LOCATION_DISCOVERED / 0xDDFC1B8F {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// sync_reload_state (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 9 bits, 0..511
        /// - `int_value`: int, 32 bits, 0..0
        /// - `int_value_2`: int, 32 bits, 0..0
        SyncReloadState / SYNC_RELOAD_STATE / 0xDE6A10DF {
            entity: int,
            arg1: int,
            int_value: int,
            int_value_2: int,
        }
    }

    rpc! {
        /// set_dematerialized (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        SetDematerialized / SET_DEMATERIALIZED / 0xDEAF79E0 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// set_player_info (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        SetPlayerInfo / SET_PLAYER_INFO / 0xE0294D95 {
            entity: int,
        }
    }

    rpc! {
        /// set_stratagem_code_completed_count (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        SetStratagemCodeCompletedCount / SET_STRATAGEM_CODE_COMPLETED_COUNT / 0xE095A8BA {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// restore_to_pickup_point (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        RestoreToPickupPoint / RESTORE_TO_PICKUP_POINT / 0xE15701F6 {
            entity: int,
        }
    }

    rpc! {
        /// request_stratagem_confirmation (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `bool_value`: bool
        RequestStratagemConfirmation / REQUEST_STRATAGEM_CONFIRMATION / 0xE15F36B2 {
            entity: int,
            vector3: vector3,
            bool_value: bool,
        }
    }

    rpc! {
        /// from_host_set_menu_screen:
        ///
        /// Arguments:
        /// - `arg0`: int, 2 bits, 0..3
        FromHostSetMenuScreen / FROM_HOST_SET_MENU_SCREEN / 0xE1E46A75 {
            arg0: int,
        }
    }

    rpc! {
        /// from_host_mission_id:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `arg1`: u64
        FromHostMissionId / FROM_HOST_MISSION_ID / 0xE25AC0A9 {
            arg0: u64,
            arg1: u64,
        }
    }

    rpc! {
        /// request_migration (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 15 bits, 0..32767
        /// - `peer_id`: u64
        RequestMigration / REQUEST_MIGRATION / 0xE29B4D18 {
            arg0: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// inflict_injury (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        InflictInjury / INFLICT_INJURY / 0xE2E86CF7 {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// objective_entity_spotted (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        ObjectiveEntitySpotted / OBJECTIVE_ENTITY_SPOTTED / 0xE337669B {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// sync_player_info:
        ///
        /// Arguments:
        /// - `network_string`: string, at most 2047 bytes
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `arg3`: u64
        /// - `arg4`: u64
        /// - `arg5`: u64
        /// - `arg6`: int, 3 bits, 0..7
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        SyncPlayerInfo / SYNC_PLAYER_INFO / 0xE36BFC38 {
            network_string: string,
            unsigned_value: int,
            unsigned_value_2: int,
            arg3: u64,
            arg4: u64,
            arg5: u64,
            arg6: int,
            unsigned_value_3: int,
        }
    }

    rpc! {
        /// from_partymember_lobby_joined:
        ///
        /// Arguments:
        /// - `arg0`: u64
        FromPartymemberLobbyJoined / FROM_PARTYMEMBER_LOBBY_JOINED / 0xE39547E9 {
            arg0: u64,
        }
    }

    rpc! {
        /// on_entity_dying (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        OnEntityDying / ON_ENTITY_DYING / 0xE3BFFB02 {
            entity: int,
        }
    }

    rpc! {
        /// distant_war_particle (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `vector3`: vector3, 31 bits over -10000..10000
        DistantWarParticle / DISTANT_WAR_PARTICLE / 0xE4162AB7 {
            unsigned_value: int,
            unsigned_value_2: int,
            vector3: vector3,
        }
    }

    rpc! {
        /// fancy_effect_system_finalise_sync (session-bound): no arguments.
        FancyEffectSystemFinaliseSync / FANCY_EFFECT_SYSTEM_FINALISE_SYNC / 0xE4211398;
    }

    rpc! {
        /// minigame_reset (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        MinigameReset / MINIGAME_RESET / 0xE42B3CC1 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// add_spawnpoint (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `entity_2`: int, 15 bits, 0..32767
        AddSpawnpoint / ADD_SPAWNPOINT / 0xE43DD0CF {
            entity: int,
            vector3: vector3,
            entity_2: int,
        }
    }

    rpc! {
        /// attach (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        Attach / ATTACH / 0xE4EEC08E {
            entity: int,
            entity_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// build_random_order (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        BuildRandomOrder / BUILD_RANDOM_ORDER / 0xE5AED405 {
            entity: int,
            unsigned_value: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// sync_galactic_war_planet_region_status:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `arg2`: int, 3 bits, 0..7
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `unsigned_value_4`: int, 32 bits, 0..0
        SyncGalacticWarPlanetRegionStatus / SYNC_GALACTIC_WAR_PLANET_REGION_STATUS / 0xE5FF9D46 {
            unsigned_value: int,
            unsigned_value_2: int,
            arg2: int,
            unsigned_value_3: int,
            float_value: float,
            bool_value: bool,
            float_value_2: float,
            unsigned_value_4: int,
        }
    }

    rpc! {
        /// peer_authorized_request (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: u64
        PeerAuthorizedRequest / PEER_AUTHORIZED_REQUEST / 0xE73ED031 {
            arg0: u64,
        }
    }

    rpc! {
        /// extract (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        Extract / EXTRACT / 0xE78513A7 {
            entity: int,
        }
    }

    rpc! {
        /// play_collision_sound (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        PlayCollisionSound / PLAY_COLLISION_SOUND / 0xE7D34A99 {
            entity: int,
            unsigned_value: int,
            float_value: float,
        }
    }

    rpc! {
        /// set_shipname (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        SetShipname / SET_SHIPNAME / 0xE7F24551 {
            unsigned_value: int,
            unsigned_value_2: int,
        }
    }

    rpc! {
        /// stratagem_ball_stratagem_failed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        StratagemBallStratagemFailed / STRATAGEM_BALL_STRATAGEM_FAILED / 0xE83EC2B2 {
            entity: int,
        }
    }

    rpc! {
        /// attachment_removed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        AttachmentRemoved / ATTACHMENT_REMOVED / 0xE87A1AF2 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// complete_wait_minigame (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        CompleteWaitMinigame / COMPLETE_WAIT_MINIGAME / 0xE8EC4C95 {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// on_hotjoined_destruction (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg1`: up to 128 x int, 8 bits, 0..255
        OnHotjoinedDestruction / ON_HOTJOINED_DESTRUCTION / 0xE99A406A {
            unsigned_value: int,
            arg1: [int; 128],
        }
    }

    rpc! {
        /// from_client_completed_loadout_weapon_swap:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: u64
        FromClientCompletedLoadoutWeaponSwap / FROM_CLIENT_COMPLETED_LOADOUT_WEAPON_SWAP / 0xE99F8C6D {
            peer_id: u64,
            arg1: u64,
        }
    }

    rpc! {
        /// trigger_particle_event_from_position_rotation (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 5 bits, 0..31
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `quaternion`: quaternion
        TriggerParticleEventFromPositionRotation / TRIGGER_PARTICLE_EVENT_FROM_POSITION_ROTATION / 0xE9B9E9AF {
            entity: int,
            arg1: int,
            vector3: vector3,
            quaternion: quaternion,
        }
    }

    rpc! {
        /// set_light_enabled (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `bool_value`: bool
        SetLightEnabled / SET_LIGHT_ENABLED / 0xEA195C36 {
            entity: int,
            id_string32: u32,
            bool_value: bool,
        }
    }

    rpc! {
        /// disable_stratagem_of_type (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 8 bits, 0..255
        DisableStratagemOfType / DISABLE_STRATAGEM_OF_TYPE / 0xEA4550DE {
            arg0: int,
        }
    }

    rpc! {
        /// shield_almost_done_charging (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ShieldAlmostDoneCharging / SHIELD_ALMOST_DONE_CHARGING / 0xEB23633C {
            entity: int,
        }
    }

    rpc! {
        /// drop_carryable (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `bool_value`: bool
        DropCarryable / DROP_CARRYABLE / 0xEB8231B8 {
            entity: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// sync_galactic_war_data_sent: no arguments.
        SyncGalacticWarDataSent / SYNC_GALACTIC_WAR_DATA_SENT / 0xED1913C9;
    }

    rpc! {
        /// voiceover_queue_entry (session-bound):
        ///
        /// Arguments:
        /// - `id_string32`: u32
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `id_string32_2`: u32
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `id_string32_3`: u32
        /// - `id_string32_4`: u32
        /// - `id_string32_5`: u32
        VoiceoverQueueEntry / VOICEOVER_QUEUE_ENTRY / 0xEEBD4E06 {
            id_string32: u32,
            unsigned_value: int,
            id_string32_2: u32,
            entity: int,
            float_value: float,
            id_string32_3: u32,
            id_string32_4: u32,
            id_string32_5: u32,
        }
    }

    rpc! {
        /// fire_single_round_advanced (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `vector3`: vector3, 31 bits over -10000..10000
        /// - `arg2`: int, 10 bits, 0..1023
        /// - `arg3`: int, 9 bits, 0..511
        /// - `id_string64`: id64, a name hash
        /// - `id_string32`: u32
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `float_value_3`: float, 31 bits over -10000..10000
        /// - `bool_value`: bool
        FireSingleRoundAdvanced / FIRE_SINGLE_ROUND_ADVANCED / 0xEF829147 {
            entity: int,
            vector3: vector3,
            arg2: int,
            arg3: int,
            id_string64: id64,
            id_string32: u32,
            float_value: float,
            float_value_2: float,
            float_value_3: float,
            bool_value: bool,
        }
    }

    rpc! {
        /// set_ragdoll_pose (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `ragdoll_pose`: int, 3 bits, 0..7
        /// - `bool_value`: bool
        SetRagdollPose / SET_RAGDOLL_POSE / 0xEF9068B0 {
            entity: int,
            ragdoll_pose: int,
            bool_value: bool,
        }
    }

    rpc! {
        /// game_object_sync_done_to (session-bound): no arguments.
        GameObjectSyncDoneTo / GAME_OBJECT_SYNC_DONE_TO / 0xF004B017;
    }

    rpc! {
        /// material_variable_vector4 (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 4 bits, 0..15
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        /// - `float_value_3`: float, 31 bits over -10000..10000
        /// - `float_value_4`: float, 31 bits over -10000..10000
        MaterialVariableVector4 / MATERIAL_VARIABLE_VECTOR4 / 0xF01B53AC {
            entity: int,
            arg1: int,
            float_value: float,
            float_value_2: float,
            float_value_3: float,
            float_value_4: float,
        }
    }

    rpc! {
        /// shield_zones_state_changed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: up to 2 x u64
        ShieldZonesStateChanged / SHIELD_ZONES_STATE_CHANGED / 0xF0777A16 {
            entity: int,
            arg1: [u64; 2],
        }
    }

    rpc! {
        /// change_ui_avatar_animation (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `float_value_2`: float, 31 bits over -10000..10000
        ChangeUiAvatarAnimation / CHANGE_UI_AVATAR_ANIMATION / 0xF24760A9 {
            unsigned_value: int,
            unsigned_value_2: int,
            float_value: float,
            float_value_2: float,
        }
    }

    rpc! {
        /// request_enter_seat_collection_denied (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        RequestEnterSeatCollectionDenied / REQUEST_ENTER_SEAT_COLLECTION_DENIED / 0xF2A7F3E4 {
            entity: int,
            entity_2: int,
        }
    }

    rpc! {
        /// synced_health_damage_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `int_value`: int, 32 bits, 0..0
        /// - `int_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value`: int, 32 bits, 0..0
        SyncedHealthDamageRequest / SYNCED_HEALTH_DAMAGE_REQUEST / 0xF2B32A25 {
            entity: int,
            int_value: int,
            int_value_2: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// select_loadout_weapon (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `arg1`: up to 16 x u32
        /// - `arg2`: up to 48 x int, 32 bits, 0..0
        /// - `arg3`: up to 48 x int, 32 bits, 0..0
        /// - `arg4`: up to 48 x int, 32 bits, 0..0
        SelectLoadoutWeapon / SELECT_LOADOUT_WEAPON / 0xF3C64D6B {
            unsigned_value: int,
            arg1: [u32; 16],
            arg2: [int; 48],
            arg3: [int; 48],
            arg4: [int; 48],
        }
    }

    rpc! {
        /// from_host_execute_synced_task:
        ///
        /// Arguments:
        /// - `synced_task`: int, 2 bits, 0..3
        FromHostExecuteSyncedTask / FROM_HOST_EXECUTE_SYNCED_TASK / 0xF3ED1929 {
            synced_task: int,
        }
    }

    rpc! {
        /// stratagem_trigger_cooldown (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        /// - `float_value`: float, 31 bits over -10000..10000
        StratagemTriggerCooldown / STRATAGEM_TRIGGER_COOLDOWN / 0xF415BB4D {
            unsigned_value: int,
            bool_value: bool,
            float_value: float,
        }
    }

    rpc! {
        /// increment_objective_counter (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `float_value`: float, 31 bits over -10000..10000
        /// - `unsigned_value`: int, 32 bits, 0..0
        IncrementObjectiveCounter / INCREMENT_OBJECTIVE_COUNTER / 0xF4380900 {
            entity: int,
            float_value: float,
            unsigned_value: int,
        }
    }

    rpc! {
        /// interaction_request (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `entity_2`: int, 15 bits, 0..32767
        /// - `arg2`: int, 4 bits, 0..15
        /// - `entity_3`: int, 15 bits, 0..32767
        InteractionRequest / INTERACTION_REQUEST / 0xF4829F1A {
            entity: int,
            entity_2: int,
            arg2: int,
            entity_3: int,
        }
    }

    rpc! {
        /// sync_galactic_war_global_event:
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `id_string32`: u32
        /// - `arg2`: up to 3 x int, 32 bits, 0..0
        /// - `arg3`: up to 32 x int, 32 bits, 0..0
        /// - `arg4`: int, 3 bits, 0..7
        /// - `arg5`: int, 3 bits, 0..7
        /// - `id_string32_2`: u32
        /// - `id_string32_3`: u32
        /// - `id_string32_4`: u32
        /// - `arg9`: u64
        SyncGalacticWarGlobalEvent / SYNC_GALACTIC_WAR_GLOBAL_EVENT / 0xF51055B2 {
            unsigned_value: int,
            id_string32: u32,
            arg2: [int; 3],
            arg3: [int; 32],
            arg4: int,
            arg5: int,
            id_string32_2: u32,
            id_string32_3: u32,
            id_string32_4: u32,
            arg9: u64,
        }
    }

    rpc! {
        /// sync_space_battle_units (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: up to 8 x int, 15 bits, 0..32767
        SyncSpaceBattleUnits / SYNC_SPACE_BATTLE_UNITS / 0xF55A2890 {
            arg0: [int; 8],
        }
    }

    rpc! {
        /// set_light_color (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        /// - `color`: vector3, 8 bits over 0..256
        SetLightColor / SET_LIGHT_COLOR / 0xF5FC6BD1 {
            entity: int,
            id_string32: u32,
            color: vector3,
        }
    }

    rpc! {
        /// mine_stepped_on (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        MineSteppedOn / MINE_STEPPED_ON / 0xF6399F97 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// stratagem_used (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `bool_value`: bool
        /// - `vector3`: vector3, 31 bits over -10000..10000
        StratagemUsed / STRATAGEM_USED / 0xF6A4244F {
            unsigned_value: int,
            bool_value: bool,
            vector3: vector3,
        }
    }

    rpc! {
        /// from_host_kicked: no arguments.
        FromHostKicked / FROM_HOST_KICKED / 0xF786372D;
    }

    rpc! {
        /// hostinfo:
        ///
        /// Arguments:
        /// - `arg0`: u64
        /// - `bool_value`: bool
        /// - `peer_id`: u64
        Hostinfo / HOSTINFO / 0xF800BF9F {
            arg0: u64,
            bool_value: bool,
            peer_id: u64,
        }
    }

    rpc! {
        /// set_minigame_keypad_value (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        /// - `unsigned_value_2`: int, 32 bits, 0..0
        /// - `unsigned_value_3`: int, 32 bits, 0..0
        SetMinigameKeypadValue / SET_MINIGAME_KEYPAD_VALUE / 0xF86A24E8 {
            entity: int,
            unsigned_value: int,
            unsigned_value_2: int,
            unsigned_value_3: int,
        }
    }

    rpc! {
        /// migrate_game_object_request (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 15 bits, 0..32767
        /// - `peer_id`: u64
        MigrateGameObjectRequest / MIGRATE_GAME_OBJECT_REQUEST / 0xF8A9D630 {
            arg0: int,
            peer_id: u64,
        }
    }

    rpc! {
        /// from_host_peer_metric_added:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `arg1`: u64
        FromHostPeerMetricAdded / FROM_HOST_PEER_METRIC_ADDED / 0xF92099C1 {
            peer_id: u64,
            arg1: u64,
        }
    }

    rpc! {
        /// on_hit_react_event (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `id_string32`: u32
        OnHitReactEvent / ON_HIT_REACT_EVENT / 0xFA6190CB {
            entity: int,
            id_string32: u32,
        }
    }

    rpc! {
        /// on_disabled_wait_completed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        OnDisabledWaitCompleted / ON_DISABLED_WAIT_COMPLETED / 0xFABCE871 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// dyndest_despawn (session-bound):
        ///
        /// Arguments:
        /// - `arg0`: int, 11 bits, 0..2047
        DyndestDespawn / DYNDEST_DESPAWN / 0xFB54DB03 {
            arg0: int,
        }
    }

    rpc! {
        /// remove_selected_drop_point (session-bound):
        ///
        /// Arguments:
        /// - `unsigned_value`: int, 32 bits, 0..0
        RemoveSelectedDropPoint / REMOVE_SELECTED_DROP_POINT / 0xFBC8E61E {
            unsigned_value: int,
        }
    }

    rpc! {
        /// cancel_aim_interpolation (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        CancelAimInterpolation / CANCEL_AIM_INTERPOLATION / 0xFC157AF4 {
            entity: int,
        }
    }

    rpc! {
        /// loadout_update_booster_availability (session-bound): no arguments.
        LoadoutUpdateBoosterAvailability / LOADOUT_UPDATE_BOOSTER_AVAILABILITY / 0xFC4AB3B0;
    }

    rpc! {
        /// stop_all_for_entity (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        StopAllForEntity / STOP_ALL_FOR_ENTITY / 0xFCA4B1A3 {
            entity: int,
        }
    }

    rpc! {
        /// clear_status_effect_immediate (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `arg1`: int, 7 bits, 0..127
        ClearStatusEffectImmediate / CLEAR_STATUS_EFFECT_IMMEDIATE / 0xFCB9FBBF {
            entity: int,
            arg1: int,
        }
    }

    rpc! {
        /// encounter_end_ping (session-bound):
        ///
        /// Arguments:
        /// - `vector3`: vector3, 31 bits over -10000..10000
        EncounterEndPing / ENCOUNTER_END_PING / 0xFD24CE01 {
            vector3: vector3,
        }
    }

    rpc! {
        /// displacement_destroyed (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        DisplacementDestroyed / DISPLACEMENT_DESTROYED / 0xFD5AC440 {
            entity: int,
        }
    }

    rpc! {
        /// exit_ship_hellpod (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        ExitShipHellpod / EXIT_SHIP_HELLPOD / 0xFD97C115 {
            entity: int,
        }
    }

    rpc! {
        /// stratagem_ball_stratagem_succeded (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        StratagemBallStratagemSucceded / STRATAGEM_BALL_STRATAGEM_SUCCEDED / 0xFDB94BB4 {
            entity: int,
        }
    }

    rpc! {
        /// fire_system_delta_cell_list_complete (session-bound): no arguments.
        FireSystemDeltaCellListComplete / FIRE_SYSTEM_DELTA_CELL_LIST_COMPLETE / 0xFDBF4246;
    }

    rpc! {
        /// drop_current_droppable_slot (session-bound):
        ///
        /// Arguments:
        /// - `entity`: int, 15 bits, 0..32767
        /// - `unsigned_value`: int, 32 bits, 0..0
        DropCurrentDroppableSlot / DROP_CURRENT_DROPPABLE_SLOT / 0xFE5BF7A3 {
            entity: int,
            unsigned_value: int,
        }
    }

    rpc! {
        /// from_host_notify_joined_peer:
        ///
        /// Arguments:
        /// - `peer_id`: u64
        /// - `network_string`: string, at most 2047 bytes
        FromHostNotifyJoinedPeer / FROM_HOST_NOTIFY_JOINED_PEER / 0xFEC79FAA {
            peer_id: u64,
            network_string: string,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hf_net::rpc;

    fn decode<M: RpcMessage>(args: Vec<RpcArg<'_>>) -> Option<M> {
        let owned: Vec<OwnedArg> = args.iter().map(OwnedArg::from_arg).collect::<Option<_>>()?;
        let mut ffi = Vec::new();
        for arg in &owned {
            arg.push_ffi(&mut ffi);
        }

        let event = RpcEvent {
            sender: 1,
            rpc: M::ID,
            args: &ffi,
        };

        M::decode(&event)
    }

    #[test]
    fn targets_match_the_header() {
        assert_eq!(TO_ALL, 0xFFFF_FFFF_FFFF_FFFF);
        assert_eq!(TO_HOST, 0xFFFF_FFFF_FFFF_FFFE);
        assert_eq!(MESSAGE_MAX, 1024);
        assert_eq!(SHARE_VALUE_MAX, 64);
        assert_eq!(SHARE_ROWS, 512);
    }

    #[test]
    fn scalar_rpc_round_trips() {
        let message = rpc::OutpostLocationDiscovered {
            entity: 3,
            entity_2: 4,
        };

        let args = message.encode().unwrap();
        assert_eq!(
            decode::<rpc::OutpostLocationDiscovered>(args),
            Some(message)
        );
    }

    #[test]
    fn string_and_array_rpc_round_trips() {
        let message = rpc::FromPartyleaderJoinLobby {
            arg0: 7,
            arg1: vec![8, 9],
            network_string: "lobby".to_string(),
            peer_id: 10,
        };

        let args = message.encode().unwrap();
        assert_eq!(decode::<rpc::FromPartyleaderJoinLobby>(args), Some(message));
    }

    #[test]
    fn overlong_arrays_are_refused() {
        let message = rpc::DamageZonesStateChanged {
            entity: 1,
            arg1: vec![2, 3, 4],
            peer_id: 5,
        };

        assert!(message.encode().is_none());
    }

    #[test]
    fn no_argument_rpcs_decode_only_their_event() {
        let empty = RpcEvent {
            sender: 1,
            rpc: rpc::FROM_HOST_KICKED,
            args: &[],
        };

        assert_eq!(
            rpc::FromHostKicked::decode(&empty),
            Some(rpc::FromHostKicked)
        );
        assert_eq!(
            rpc::FromHostKicked::decode(&RpcEvent {
                rpc: rpc::LOADOUT_UPDATE_BOOSTER_AVAILABILITY,
                ..empty
            }),
            None
        );
    }

    #[test]
    fn string_array_rpc_round_trips() {
        let message = rpc::ArcadeStratagemUpdatedHighscores {
            arg0: vec!["a".to_string(), "b".to_string()],
            arg1: vec![1, 2],
        };

        let args = message.encode().unwrap();
        assert_eq!(
            decode::<rpc::ArcadeStratagemUpdatedHighscores>(args),
            Some(message)
        );
    }

    #[test]
    fn vector_array_rpc_round_trips() {
        let message = rpc::StartFollowCurve {
            entity: 1,
            unsigned_value: 2,
            float_value: 3.0,
            unsigned_value_2: 4,
            arg4: vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
            arg5: vec![7.0, 8.0],
        };

        let args = message.encode().unwrap();
        assert_eq!(decode::<rpc::StartFollowCurve>(args), Some(message));
    }

    #[test]
    fn reader_rejects_a_missing_argument() {
        let event = RpcEvent {
            sender: 1,
            rpc: rpc::OUTPOST_LOCATION_DISCOVERED,
            args: &[],
        };

        assert_eq!(rpc::OutpostLocationDiscovered::decode(&event), None);
    }
}
