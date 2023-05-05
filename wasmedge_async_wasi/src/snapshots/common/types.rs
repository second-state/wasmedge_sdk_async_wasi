// bindgen --impl-debug --size_t-is-usize --no-layout-tests --no-doc-comments  --allowlist-type="__wasi.*" --default-enum-style moduleconsts WasmEdge/thirdparty/wasi/api.hpp -o wasi_types.rs

/* automatically generated by rust-bindgen 0.60.1 */

pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type const_uint8_t_ptr = u32;
pub type uint8_t_ptr = u32;
pub type __wasi_size_t = u32;
pub type __wasi_filesize_t = u64;
pub type __wasi_timestamp_t = u64;
pub mod __wasi_clockid_t {
    pub type Type = u32;
    pub const __WASI_CLOCKID_REALTIME: Type = 0;
    pub const __WASI_CLOCKID_MONOTONIC: Type = 1;
    pub const __WASI_CLOCKID_PROCESS_CPUTIME_ID: Type = 2;
    pub const __WASI_CLOCKID_THREAD_CPUTIME_ID: Type = 3;
}
pub mod __wasi_errno_t {
    pub type Type = u16;
    pub const __WASI_ERRNO_SUCCESS: Type = 0;
    pub const __WASI_ERRNO_2BIG: Type = 1;
    pub const __WASI_ERRNO_ACCES: Type = 2;
    pub const __WASI_ERRNO_ADDRINUSE: Type = 3;
    pub const __WASI_ERRNO_ADDRNOTAVAIL: Type = 4;
    pub const __WASI_ERRNO_AFNOSUPPORT: Type = 5;
    pub const __WASI_ERRNO_AGAIN: Type = 6;
    pub const __WASI_ERRNO_ALREADY: Type = 7;
    pub const __WASI_ERRNO_BADF: Type = 8;
    pub const __WASI_ERRNO_BADMSG: Type = 9;
    pub const __WASI_ERRNO_BUSY: Type = 10;
    pub const __WASI_ERRNO_CANCELED: Type = 11;
    pub const __WASI_ERRNO_CHILD: Type = 12;
    pub const __WASI_ERRNO_CONNABORTED: Type = 13;
    pub const __WASI_ERRNO_CONNREFUSED: Type = 14;
    pub const __WASI_ERRNO_CONNRESET: Type = 15;
    pub const __WASI_ERRNO_DEADLK: Type = 16;
    pub const __WASI_ERRNO_DESTADDRREQ: Type = 17;
    pub const __WASI_ERRNO_DOM: Type = 18;
    pub const __WASI_ERRNO_DQUOT: Type = 19;
    pub const __WASI_ERRNO_EXIST: Type = 20;
    pub const __WASI_ERRNO_FAULT: Type = 21;
    pub const __WASI_ERRNO_FBIG: Type = 22;
    pub const __WASI_ERRNO_HOSTUNREACH: Type = 23;
    pub const __WASI_ERRNO_IDRM: Type = 24;
    pub const __WASI_ERRNO_ILSEQ: Type = 25;
    pub const __WASI_ERRNO_INPROGRESS: Type = 26;
    pub const __WASI_ERRNO_INTR: Type = 27;
    pub const __WASI_ERRNO_INVAL: Type = 28;
    pub const __WASI_ERRNO_IO: Type = 29;
    pub const __WASI_ERRNO_ISCONN: Type = 30;
    pub const __WASI_ERRNO_ISDIR: Type = 31;
    pub const __WASI_ERRNO_LOOP: Type = 32;
    pub const __WASI_ERRNO_MFILE: Type = 33;
    pub const __WASI_ERRNO_MLINK: Type = 34;
    pub const __WASI_ERRNO_MSGSIZE: Type = 35;
    pub const __WASI_ERRNO_MULTIHOP: Type = 36;
    pub const __WASI_ERRNO_NAMETOOLONG: Type = 37;
    pub const __WASI_ERRNO_NETDOWN: Type = 38;
    pub const __WASI_ERRNO_NETRESET: Type = 39;
    pub const __WASI_ERRNO_NETUNREACH: Type = 40;
    pub const __WASI_ERRNO_NFILE: Type = 41;
    pub const __WASI_ERRNO_NOBUFS: Type = 42;
    pub const __WASI_ERRNO_NODEV: Type = 43;
    pub const __WASI_ERRNO_NOENT: Type = 44;
    pub const __WASI_ERRNO_NOEXEC: Type = 45;
    pub const __WASI_ERRNO_NOLCK: Type = 46;
    pub const __WASI_ERRNO_NOLINK: Type = 47;
    pub const __WASI_ERRNO_NOMEM: Type = 48;
    pub const __WASI_ERRNO_NOMSG: Type = 49;
    pub const __WASI_ERRNO_NOPROTOOPT: Type = 50;
    pub const __WASI_ERRNO_NOSPC: Type = 51;
    pub const __WASI_ERRNO_NOSYS: Type = 52;
    pub const __WASI_ERRNO_NOTCONN: Type = 53;
    pub const __WASI_ERRNO_NOTDIR: Type = 54;
    pub const __WASI_ERRNO_NOTEMPTY: Type = 55;
    pub const __WASI_ERRNO_NOTRECOVERABLE: Type = 56;
    pub const __WASI_ERRNO_NOTSOCK: Type = 57;
    pub const __WASI_ERRNO_NOTSUP: Type = 58;
    pub const __WASI_ERRNO_NOTTY: Type = 59;
    pub const __WASI_ERRNO_NXIO: Type = 60;
    pub const __WASI_ERRNO_OVERFLOW: Type = 61;
    pub const __WASI_ERRNO_OWNERDEAD: Type = 62;
    pub const __WASI_ERRNO_PERM: Type = 63;
    pub const __WASI_ERRNO_PIPE: Type = 64;
    pub const __WASI_ERRNO_PROTO: Type = 65;
    pub const __WASI_ERRNO_PROTONOSUPPORT: Type = 66;
    pub const __WASI_ERRNO_PROTOTYPE: Type = 67;
    pub const __WASI_ERRNO_RANGE: Type = 68;
    pub const __WASI_ERRNO_ROFS: Type = 69;
    pub const __WASI_ERRNO_SPIPE: Type = 70;
    pub const __WASI_ERRNO_SRCH: Type = 71;
    pub const __WASI_ERRNO_STALE: Type = 72;
    pub const __WASI_ERRNO_TIMEDOUT: Type = 73;
    pub const __WASI_ERRNO_TXTBSY: Type = 74;
    pub const __WASI_ERRNO_XDEV: Type = 75;
    pub const __WASI_ERRNO_NOTCAPABLE: Type = 76;
    pub const __WASI_ERRNO_AIADDRFAMILY: Type = 77;
    pub const __WASI_ERRNO_AIAGAIN: Type = 78;
    pub const __WASI_ERRNO_AIBADFLAG: Type = 79;
    pub const __WASI_ERRNO_AIFAIL: Type = 80;
    pub const __WASI_ERRNO_AIFAMILY: Type = 81;
    pub const __WASI_ERRNO_AIMEMORY: Type = 82;
    pub const __WASI_ERRNO_AINODATA: Type = 83;
    pub const __WASI_ERRNO_AINONAME: Type = 84;
    pub const __WASI_ERRNO_AISERVICE: Type = 85;
    pub const __WASI_ERRNO_AISOCKTYPE: Type = 86;
    pub const __WASI_ERRNO_AISYSTEM: Type = 87;
}
pub mod __wasi_rights_t {
    pub type Type = u64;
    pub const __WASI_RIGHTS_FD_DATASYNC: Type = 1;
    pub const __WASI_RIGHTS_FD_READ: Type = 2;
    pub const __WASI_RIGHTS_FD_SEEK: Type = 4;
    pub const __WASI_RIGHTS_FD_FDSTAT_SET_FLAGS: Type = 8;
    pub const __WASI_RIGHTS_FD_SYNC: Type = 16;
    pub const __WASI_RIGHTS_FD_TELL: Type = 32;
    pub const __WASI_RIGHTS_FD_WRITE: Type = 64;
    pub const __WASI_RIGHTS_FD_ADVISE: Type = 128;
    pub const __WASI_RIGHTS_FD_ALLOCATE: Type = 256;
    pub const __WASI_RIGHTS_PATH_CREATE_DIRECTORY: Type = 512;
    pub const __WASI_RIGHTS_PATH_CREATE_FILE: Type = 1024;
    pub const __WASI_RIGHTS_PATH_LINK_SOURCE: Type = 2048;
    pub const __WASI_RIGHTS_PATH_LINK_TARGET: Type = 4096;
    pub const __WASI_RIGHTS_PATH_OPEN: Type = 8192;
    pub const __WASI_RIGHTS_FD_READDIR: Type = 16384;
    pub const __WASI_RIGHTS_PATH_READLINK: Type = 32768;
    pub const __WASI_RIGHTS_PATH_RENAME_SOURCE: Type = 65536;
    pub const __WASI_RIGHTS_PATH_RENAME_TARGET: Type = 131072;
    pub const __WASI_RIGHTS_PATH_FILESTAT_GET: Type = 262144;
    pub const __WASI_RIGHTS_PATH_FILESTAT_SET_SIZE: Type = 524288;
    pub const __WASI_RIGHTS_PATH_FILESTAT_SET_TIMES: Type = 1048576;
    pub const __WASI_RIGHTS_FD_FILESTAT_GET: Type = 2097152;
    pub const __WASI_RIGHTS_FD_FILESTAT_SET_SIZE: Type = 4194304;
    pub const __WASI_RIGHTS_FD_FILESTAT_SET_TIMES: Type = 8388608;
    pub const __WASI_RIGHTS_PATH_SYMLINK: Type = 16777216;
    pub const __WASI_RIGHTS_PATH_REMOVE_DIRECTORY: Type = 33554432;
    pub const __WASI_RIGHTS_PATH_UNLINK_FILE: Type = 67108864;
    pub const __WASI_RIGHTS_POLL_FD_READWRITE: Type = 134217728;
    pub const __WASI_RIGHTS_SOCK_SHUTDOWN: Type = 268435456;
    pub const __WASI_RIGHTS_SOCK_OPEN: Type = 536870912;
    pub const __WASI_RIGHTS_SOCK_CLOSE: Type = 1073741824;
    pub const __WASI_RIGHTS_SOCK_BIND: Type = 2147483648;
    pub const __WASI_RIGHTS_SOCK_RECV: Type = 4294967296;
    pub const __WASI_RIGHTS_SOCK_RECV_FROM: Type = 8589934592;
    pub const __WASI_RIGHTS_SOCK_SEND: Type = 17179869184;
    pub const __WASI_RIGHTS_SOCK_SEND_TO: Type = 34359738368;
}
pub type __wasi_fd_t = i32;
pub type __wasi_sock_d_t = __wasi_fd_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_iovec_t {
    pub buf: uint8_t_ptr,
    pub buf_len: __wasi_size_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_ciovec_t {
    pub buf: const_uint8_t_ptr,
    pub buf_len: __wasi_size_t,
}
pub type __wasi_filedelta_t = i64;
pub mod __wasi_whence_t {
    pub type Type = u8;
    pub const __WASI_WHENCE_SET: Type = 0;
    pub const __WASI_WHENCE_CUR: Type = 1;
    pub const __WASI_WHENCE_END: Type = 2;
}
pub type __wasi_dircookie_t = u64;
pub type __wasi_dirnamlen_t = u32;
pub type __wasi_inode_t = u64;
pub mod __wasi_filetype_t {
    pub type Type = u8;
    pub const __WASI_FILETYPE_UNKNOWN: Type = 0;
    pub const __WASI_FILETYPE_BLOCK_DEVICE: Type = 1;
    pub const __WASI_FILETYPE_CHARACTER_DEVICE: Type = 2;
    pub const __WASI_FILETYPE_DIRECTORY: Type = 3;
    pub const __WASI_FILETYPE_REGULAR_FILE: Type = 4;
    pub const __WASI_FILETYPE_SOCKET_DGRAM: Type = 5;
    pub const __WASI_FILETYPE_SOCKET_STREAM: Type = 6;
    pub const __WASI_FILETYPE_SYMBOLIC_LINK: Type = 7;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_dirent_t {
    pub d_next: __wasi_dircookie_t,
    pub d_ino: __wasi_inode_t,
    pub d_namlen: __wasi_dirnamlen_t,
    pub d_type: __wasi_filetype_t::Type,
}
pub mod __wasi_advice_t {
    pub type Type = u8;
    pub const __WASI_ADVICE_NORMAL: Type = 0;
    pub const __WASI_ADVICE_SEQUENTIAL: Type = 1;
    pub const __WASI_ADVICE_RANDOM: Type = 2;
    pub const __WASI_ADVICE_WILLNEED: Type = 3;
    pub const __WASI_ADVICE_DONTNEED: Type = 4;
    pub const __WASI_ADVICE_NOREUSE: Type = 5;
}
pub mod __wasi_fdflags_t {
    pub type Type = u16;
    pub const __WASI_FDFLAGS_APPEND: Type = 1;
    pub const __WASI_FDFLAGS_DSYNC: Type = 2;
    pub const __WASI_FDFLAGS_NONBLOCK: Type = 4;
    pub const __WASI_FDFLAGS_RSYNC: Type = 8;
    pub const __WASI_FDFLAGS_SYNC: Type = 16;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_fdstat_t {
    pub fs_filetype: __wasi_filetype_t::Type,
    pub fs_flags: __wasi_fdflags_t::Type,
    pub fs_rights_base: __wasi_rights_t::Type,
    pub fs_rights_inheriting: __wasi_rights_t::Type,
}
pub type __wasi_device_t = u64;
pub mod __wasi_fstflags_t {
    pub type Type = u16;
    pub const __WASI_FSTFLAGS_ATIM: Type = 1;
    pub const __WASI_FSTFLAGS_ATIM_NOW: Type = 2;
    pub const __WASI_FSTFLAGS_MTIM: Type = 4;
    pub const __WASI_FSTFLAGS_MTIM_NOW: Type = 8;
}
pub mod __wasi_lookupflags_t {
    pub type Type = u32;
    pub const __WASI_LOOKUPFLAGS_SYMLINK_FOLLOW: Type = 1;
}
pub mod __wasi_oflags_t {
    pub type Type = u16;
    pub const __WASI_OFLAGS_CREAT: Type = 1;
    pub const __WASI_OFLAGS_DIRECTORY: Type = 2;
    pub const __WASI_OFLAGS_EXCL: Type = 4;
    pub const __WASI_OFLAGS_TRUNC: Type = 8;
}
pub type __wasi_linkcount_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_filestat_t {
    pub dev: __wasi_device_t,
    pub ino: __wasi_inode_t,
    pub filetype: __wasi_filetype_t::Type,
    pub nlink: __wasi_linkcount_t,
    pub size: __wasi_filesize_t,
    pub atim: __wasi_timestamp_t,
    pub mtim: __wasi_timestamp_t,
    pub ctim: __wasi_timestamp_t,
}
pub type __wasi_userdata_t = u64;
pub mod __wasi_eventtype_t {
    pub type Type = u8;
    pub const __WASI_EVENTTYPE_CLOCK: Type = 0;
    pub const __WASI_EVENTTYPE_FD_READ: Type = 1;
    pub const __WASI_EVENTTYPE_FD_WRITE: Type = 2;
}
pub mod __wasi_eventrwflags_t {
    pub type Type = u16;
    pub const __WASI_EVENTRWFLAGS_FD_READWRITE_HANGUP: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_event_fd_readwrite_t {
    pub nbytes: __wasi_filesize_t,
    pub flags: __wasi_eventrwflags_t::Type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_event_t {
    pub userdata: __wasi_userdata_t,
    pub error: __wasi_errno_t::Type,
    pub type_: __wasi_eventtype_t::Type,
    pub fd_readwrite: __wasi_event_fd_readwrite_t,
}
pub mod __wasi_subclockflags_t {
    pub type Type = u16;
    pub const __WASI_SUBCLOCKFLAGS_SUBSCRIPTION_CLOCK_ABSTIME: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_subscription_clock_t {
    pub id: __wasi_clockid_t::Type,
    pub timeout: __wasi_timestamp_t,
    pub precision: __wasi_timestamp_t,
    pub flags: __wasi_subclockflags_t::Type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_subscription_fd_readwrite_t {
    pub file_descriptor: __wasi_fd_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __wasi_subscription_u_u_t {
    pub clock: __wasi_subscription_clock_t,
    pub fd_read: __wasi_subscription_fd_readwrite_t,
    pub fd_write: __wasi_subscription_fd_readwrite_t,
}
impl ::std::fmt::Debug for __wasi_subscription_u_u_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "__wasi_subscription_u_u_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_subscription_u_t {
    pub tag: __wasi_eventtype_t::Type,
    pub u: __wasi_subscription_u_u_t,
}
impl ::std::fmt::Debug for __wasi_subscription_u_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "__wasi_subscription_u_t {{ tag: {:?}, u: {:?} }}",
            self.tag, self.u
        )
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_subscription_t {
    pub userdata: __wasi_userdata_t,
    pub u: __wasi_subscription_u_t,
}
impl ::std::fmt::Debug for __wasi_subscription_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "__wasi_subscription_t {{ userdata: {:?}, u: {:?} }}",
            self.userdata, self.u
        )
    }
}
pub type __wasi_exitcode_t = u32;
pub mod __wasi_signal_t {
    pub type Type = u8;
    pub const __WASI_SIGNAL_NONE: Type = 0;
    pub const __WASI_SIGNAL_HUP: Type = 1;
    pub const __WASI_SIGNAL_INT: Type = 2;
    pub const __WASI_SIGNAL_QUIT: Type = 3;
    pub const __WASI_SIGNAL_ILL: Type = 4;
    pub const __WASI_SIGNAL_TRAP: Type = 5;
    pub const __WASI_SIGNAL_ABRT: Type = 6;
    pub const __WASI_SIGNAL_BUS: Type = 7;
    pub const __WASI_SIGNAL_FPE: Type = 8;
    pub const __WASI_SIGNAL_KILL: Type = 9;
    pub const __WASI_SIGNAL_USR1: Type = 10;
    pub const __WASI_SIGNAL_SEGV: Type = 11;
    pub const __WASI_SIGNAL_USR2: Type = 12;
    pub const __WASI_SIGNAL_PIPE: Type = 13;
    pub const __WASI_SIGNAL_ALRM: Type = 14;
    pub const __WASI_SIGNAL_TERM: Type = 15;
    pub const __WASI_SIGNAL_CHLD: Type = 16;
    pub const __WASI_SIGNAL_CONT: Type = 17;
    pub const __WASI_SIGNAL_STOP: Type = 18;
    pub const __WASI_SIGNAL_TSTP: Type = 19;
    pub const __WASI_SIGNAL_TTIN: Type = 20;
    pub const __WASI_SIGNAL_TTOU: Type = 21;
    pub const __WASI_SIGNAL_URG: Type = 22;
    pub const __WASI_SIGNAL_XCPU: Type = 23;
    pub const __WASI_SIGNAL_XFSZ: Type = 24;
    pub const __WASI_SIGNAL_VTALRM: Type = 25;
    pub const __WASI_SIGNAL_PROF: Type = 26;
    pub const __WASI_SIGNAL_WINCH: Type = 27;
    pub const __WASI_SIGNAL_POLL: Type = 28;
    pub const __WASI_SIGNAL_PWR: Type = 29;
    pub const __WASI_SIGNAL_SYS: Type = 30;
}
pub mod __wasi_address_family_t {
    pub type Type = u8;
    pub const __WASI_ADDRESS_FAMILY_UNSPEC: Type = 0;
    pub const __WASI_ADDRESS_FAMILY_INET4: Type = 1;
    pub const __WASI_ADDRESS_FAMILY_INET6: Type = 2;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_address_t {
    pub buf: uint8_t_ptr,
    pub buf_len: __wasi_size_t,
}
pub mod __wasi_sock_opt_level_t {
    pub type Type = u32;
    pub const __WASI_SOCK_OPT_LEVEL_SOL_SOCKET: Type = 0;
}
pub mod __wasi_sock_opt_so_t {
    pub type Type = u32;
    pub const __WASI_SOCK_OPT_SO_REUSEADDR: Type = 0;
    pub const __WASI_SOCK_OPT_SO_TYPE: Type = 1;
    pub const __WASI_SOCK_OPT_SO_ERROR: Type = 2;
    pub const __WASI_SOCK_OPT_SO_DONTROUTE: Type = 3;
    pub const __WASI_SOCK_OPT_SO_BROADCAST: Type = 4;
    pub const __WASI_SOCK_OPT_SO_SNDBUF: Type = 5;
    pub const __WASI_SOCK_OPT_SO_RCVBUF: Type = 6;
    pub const __WASI_SOCK_OPT_SO_KEEPALIVE: Type = 7;
    pub const __WASI_SOCK_OPT_SO_OOBINLINE: Type = 8;
    pub const __WASI_SOCK_OPT_SO_LINGER: Type = 9;
    pub const __WASI_SOCK_OPT_SO_RCVLOWAT: Type = 10;
    pub const __WASI_SOCK_OPT_SO_RCVTIMEO: Type = 11;
    pub const __WASI_SOCK_OPT_SO_SNDTIMEO: Type = 12;
    pub const __WASI_SOCK_OPT_SO_ACCEPTCONN: Type = 13;
    pub const __WASI_SOCK_OPT_SO_BINDTODEVICE: Type = 14;
}
pub mod __wasi_aiflags_t {
    pub type Type = u16;
    pub const __WASI_AIFLAGS_AI_PASSIVE: Type = 1;
    pub const __WASI_AIFLAGS_AI_CANONNAME: Type = 2;
    pub const __WASI_AIFLAGS_AI_NUMERICHOST: Type = 4;
    pub const __WASI_AIFLAGS_AI_NUMERICSERV: Type = 8;
    pub const __WASI_AIFLAGS_AI_V4MAPPED: Type = 16;
    pub const __WASI_AIFLAGS_AI_ALL: Type = 32;
    pub const __WASI_AIFLAGS_AI_ADDRCONFIG: Type = 64;
}
pub mod __wasi_sock_type_t {
    pub type Type = u8;
    pub const __WASI_SOCK_TYPE_SOCK_ANY: Type = 0;
    pub const __WASI_SOCK_TYPE_SOCK_DGRAM: Type = 1;
    pub const __WASI_SOCK_TYPE_SOCK_STREAM: Type = 2;
}
pub mod __wasi_protocol_t {
    pub type Type = u8;
    pub const __WASI_PROTOCOL_IPPROTO_IP: Type = 0;
    pub const __WASI_PROTOCOL_IPPROTO_TCP: Type = 1;
    pub const __WASI_PROTOCOL_IPPROTO_UDP: Type = 2;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_sockaddr_in_t {
    pub sin_family: __wasi_address_family_t::Type,
    pub sin_port: u16,
    pub sin_addr: __wasi_address_t,
    pub sin_zero_len: __wasi_size_t,
    pub sin_zero: uint8_t_ptr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_sockaddr_t {
    pub sa_family: __wasi_address_family_t::Type,
    pub sa_data_len: __wasi_size_t,
    pub sa_data: uint8_t_ptr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_addrinfo_t {
    pub ai_flags: __wasi_aiflags_t::Type,
    pub ai_family: __wasi_address_family_t::Type,
    pub ai_socktype: __wasi_sock_type_t::Type,
    pub ai_protocol: __wasi_protocol_t::Type,
    pub ai_addrlen: __wasi_size_t,
    pub ai_addr: uint8_t_ptr,
    pub ai_canonname: uint8_t_ptr,
    pub ai_canonname_len: __wasi_size_t,
    pub ai_next: uint8_t_ptr,
}
pub mod __wasi_riflags_t {
    pub type Type = u16;
    pub const __WASI_RIFLAGS_RECV_PEEK: Type = 1;
    pub const __WASI_RIFLAGS_RECV_WAITALL: Type = 2;
}
pub mod __wasi_roflags_t {
    pub type Type = u16;
    pub const __WASI_ROFLAGS_RECV_DATA_TRUNCATED: Type = 1;
}
pub type __wasi_siflags_t = u16;
pub mod __wasi_sdflags_t {
    pub type Type = u8;
    pub const __WASI_SDFLAGS_RD: Type = 1;
    pub const __WASI_SDFLAGS_WR: Type = 2;
}
pub mod __wasi_preopentype_t {
    pub type Type = u8;
    pub const __WASI_PREOPENTYPE_DIR: Type = 0;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __wasi_prestat_dir_t {
    pub pr_name_len: __wasi_size_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __wasi_prestat_u_t {
    pub dir: __wasi_prestat_dir_t,
}
impl ::std::fmt::Debug for __wasi_prestat_u_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "__wasi_prestat_u_t {{ union }}")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_prestat_t {
    pub tag: __wasi_preopentype_t::Type,
    pub u: __wasi_prestat_u_t,
}
impl ::std::fmt::Debug for __wasi_prestat_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "__wasi_prestat_t {{ tag: {:?}, u: {:?} }}",
            self.tag, self.u
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wasi_timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}
