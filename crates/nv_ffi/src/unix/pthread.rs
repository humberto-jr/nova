//
// sched.h
//

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct sched_param {
	pub sched_priority: i32,
}

//
// pthread.h
//

pub type pthread_t = u64;

pub type pthread_key_t = u32;

pub type pthread_once_t = i32;

pub type pthread_spinlock_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
	pub size: [i8; 4],
	pub align: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
	pub size: [i8; 4],
	pub align: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
	pub size: [i8; 56],
	pub align: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
	pub data: __pthread_mutex_s,
	pub size: [i8; 40],
	pub align: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
	pub data: __pthread_cond_s,
	pub size: [i8; 48],
	pub align: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
	pub data: __pthread_rwlock_arch_t,
	pub size: [i8; 56],
	pub align: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
	pub size: [i8; 8],
	pub align: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
	pub size: [i8; 32],
	pub align: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
	pub size: [i8; 4],
	pub align: i32,
}

// NOTE: The StartRoutine, InitRoutine and DestrFunction symbols used
// below are not part of the original library.
pub type StartRoutine = extern "C" fn(arg: *mut ()) -> *mut ();
pub type InitRoutine = extern "C" fn();
pub type DestrFunction = extern "C" fn(arg1: *mut ());

#[link(name = "pthread")]
unsafe extern "C" {
	pub fn pthread_create(thread: *mut pthread_t, attr: *const pthread_attr_t, start_routine: StartRoutine, arg: *mut ()) -> i32;

	pub fn pthread_exit(retval: *mut ());

	pub fn pthread_join(thread: pthread_t, retval: *mut *mut ()) -> i32;

	pub fn pthread_detach(thread: pthread_t) -> i32;

	pub fn pthread_self() -> pthread_t;

	pub fn pthread_equal(thread1: pthread_t, thread2: pthread_t) -> i32;

	pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> i32;

	pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> i32;

	pub fn pthread_attr_getdetachstate(attr: *const pthread_attr_t, detachstate: *mut i32) -> i32;

	pub fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: i32) -> i32;

	pub fn pthread_attr_getguardsize(attr: *const pthread_attr_t, guardsize: *mut usize) -> i32;

	pub fn pthread_attr_setguardsize(attr: *mut pthread_attr_t, guardsize: usize) -> i32;

	pub fn pthread_attr_getschedparam(attr: *const pthread_attr_t, param: *mut sched_param) -> i32;

	pub fn pthread_attr_setschedparam(attr: *mut pthread_attr_t, param: *const sched_param) -> i32;

	pub fn pthread_attr_getschedpolicy(attr: *const pthread_attr_t, policy: *mut i32) -> i32;

	pub fn pthread_attr_setschedpolicy(attr: *mut pthread_attr_t, policy: i32) -> i32;

	pub fn pthread_attr_getinheritsched(attr: *const pthread_attr_t, inherit: *mut i32) -> i32;

	pub fn pthread_attr_setinheritsched(attr: *mut pthread_attr_t, inherit: i32) -> i32;

	pub fn pthread_attr_getscope(attr: *const pthread_attr_t, scope: *mut i32) -> i32;

	pub fn pthread_attr_setscope(attr: *mut pthread_attr_t, scope: i32) -> i32;

	pub fn pthread_attr_getstackaddr(attr: *const pthread_attr_t, stackaddr: *mut *mut ()) -> i32;

	pub fn pthread_attr_setstackaddr(attr: *mut pthread_attr_t, stackaddr: *mut ()) -> i32;

	pub fn pthread_attr_getstacksize(attr: *const pthread_attr_t, stacksize: *mut usize) -> i32;

	pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stacksize: usize) -> i32;

	pub fn pthread_attr_getstack(attr: *const pthread_attr_t, stackaddr: *mut *mut (), stacksize: *mut usize) -> i32;

	pub fn pthread_attr_setstack(attr: *mut pthread_attr_t, stackaddr: *mut (), stacksize: usize) -> i32;

	pub fn pthread_setschedparam(target_thread: pthread_t, policy: i32, param: *const sched_param) -> i32;

	pub fn pthread_getschedparam(target_thread: pthread_t, policy: *mut i32, param: *mut sched_param) -> i32;

	pub fn pthread_setschedprio(target_thread: pthread_t, prio: i32) -> i32;

	pub fn pthread_once(once_control: *mut pthread_once_t, init_routine: InitRoutine) -> i32;

	pub fn pthread_setcancelstate(state: i32, oldstate: *mut i32) -> i32;

	pub fn pthread_setcanceltype(ty: i32, oldtype: *mut i32) -> i32;

	pub fn pthread_cancel(thread: pthread_t) -> i32;

	pub fn pthread_testcancel();

	pub fn pthread_mutex_init(mutex: *mut pthread_mutex_t, mutexattr: *const pthread_mutexattr_t) -> i32;

	pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> i32;

	pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> i32;

	pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> i32;

	//	pub fn pthread_mutex_timedlock(mutex: *mut pthread_mutex_t, abstime: *const timespec) -> i32;

	pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> i32;

	pub fn pthread_mutex_getprioceiling(mutex: *const pthread_mutex_t, prioceiling: *mut i32) -> i32;

	pub fn pthread_mutex_setprioceiling(mutex: *mut pthread_mutex_t, prioceiling: i32, old_ceiling: *mut i32) -> i32;

	pub fn pthread_mutex_consistent(mutex: *mut pthread_mutex_t) -> i32;

	pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> i32;

	pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> i32;

	pub fn pthread_mutexattr_getpshared(attr: *const pthread_mutexattr_t, pshared: *mut i32) -> i32;

	pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, pshared: i32) -> i32;

	pub fn pthread_mutexattr_gettype(attr: *const pthread_mutexattr_t, kind: *mut i32) -> i32;

	pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, kind: i32) -> i32;

	pub fn pthread_mutexattr_getprotocol(attr: *const pthread_mutexattr_t, protocol: *mut i32) -> i32;

	pub fn pthread_mutexattr_setprotocol(attr: *mut pthread_mutexattr_t, protocol: i32) -> i32;

	pub fn pthread_mutexattr_getprioceiling(attr: *const pthread_mutexattr_t, prioceiling: *mut i32) -> i32;

	pub fn pthread_mutexattr_setprioceiling(attr: *mut pthread_mutexattr_t, prioceiling: i32) -> i32;

	pub fn pthread_mutexattr_getrobust(attr: *const pthread_mutexattr_t, robustness: *mut i32) -> i32;

	pub fn pthread_mutexattr_setrobust(attr: *mut pthread_mutexattr_t, robustness: i32) -> i32;

	pub fn pthread_rwlock_init(rwlock: *mut pthread_rwlock_t, attr: *const pthread_rwlockattr_t) -> i32;

	pub fn pthread_rwlock_destroy(rwlock: *mut pthread_rwlock_t) -> i32;

	pub fn pthread_rwlock_rdlock(rwlock: *mut pthread_rwlock_t) -> i32;

	pub fn pthread_rwlock_tryrdlock(rwlock: *mut pthread_rwlock_t) -> i32;

	//	pub fn pthread_rwlock_timedrdlock(rwlock: *mut pthread_rwlock_t, abstime: *const timespec) -> i32;

	pub fn pthread_rwlock_wrlock(rwlock: *mut pthread_rwlock_t) -> i32;

	pub fn pthread_rwlock_trywrlock(rwlock: *mut pthread_rwlock_t) -> i32;

	//	pub fn pthread_rwlock_timedwrlock(rwlock: *mut pthread_rwlock_t, abstime: *const timespec) -> i32;

	pub fn pthread_rwlock_unlock(rwlock: *mut pthread_rwlock_t) -> i32;

	pub fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> i32;

	pub fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> i32;

	pub fn pthread_rwlockattr_getpshared(attr: *const pthread_rwlockattr_t, pshared: *mut i32) -> i32;

	pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, pshared: i32) -> i32;

	pub fn pthread_rwlockattr_getkind_np(attr: *const pthread_rwlockattr_t, pref: *mut i32) -> i32;

	pub fn pthread_rwlockattr_setkind_np(attr: *mut pthread_rwlockattr_t, pref: i32) -> i32;

	pub fn pthread_cond_init(cond: *mut pthread_cond_t, cond_attr: *const pthread_condattr_t) -> i32;

	pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> i32;

	pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> i32;

	pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> i32;

	pub fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> i32;

	//	pub fn pthread_cond_timedwait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t, abstime: *const timespec) -> i32;

	pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> i32;

	pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> i32;

	pub fn pthread_condattr_getpshared(attr: *const pthread_condattr_t, pshared: *mut i32) -> i32;

	pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: i32) -> i32;

	//	pub fn pthread_condattr_getclock(attr: *const pthread_condattr_t, clock_id: *mut __clockid_t) -> i32;

	//	pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock_id: __clockid_t) -> i32;

	pub fn pthread_spin_init(lock: *mut pthread_spinlock_t, pshared: i32) -> i32;

	pub fn pthread_spin_destroy(lock: *mut pthread_spinlock_t) -> i32;

	pub fn pthread_spin_lock(lock: *mut pthread_spinlock_t) -> i32;

	pub fn pthread_spin_trylock(lock: *mut pthread_spinlock_t) -> i32;

	pub fn pthread_spin_unlock(lock: *mut pthread_spinlock_t) -> i32;

	pub fn pthread_barrier_init(barrier: *mut pthread_barrier_t, attr: *const pthread_barrierattr_t, count: u32) -> i32;

	pub fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> i32;

	pub fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> i32;

	pub fn pthread_barrierattr_init(attr: *mut pthread_barrierattr_t) -> i32;

	pub fn pthread_barrierattr_destroy(attr: *mut pthread_barrierattr_t) -> i32;

	pub fn pthread_barrierattr_getpshared(attr: *const pthread_barrierattr_t, pshared: *mut i32) -> i32;

	pub fn pthread_barrierattr_setpshared(attr: *mut pthread_barrierattr_t, pshared: i32) -> i32;

	pub fn pthread_key_create(key: *mut pthread_key_t, destr_function: DestrFunction) -> i32;

	pub fn pthread_key_delete(key: pthread_key_t) -> i32;

	pub fn pthread_getspecific(key: pthread_key_t) -> *mut ();

	pub fn pthread_setspecific(key: pthread_key_t, pointer: *const ()) -> i32;

	//	pub fn pthread_getcpuclockid(thread_id: pthread_t, clock_id: *mut __clockid_t) -> i32;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __pthread_internal_list {
	pub __prev: *mut __pthread_internal_list,
	pub __next: *mut __pthread_internal_list,
}

pub type __pthread_list_t = __pthread_internal_list;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __pthread_mutex_s {
	pub __lock: i32,
	pub __count: u32,
	pub __owner: i32,
	pub __nusers: u32,
	pub __kind: i32,
	pub __spins: i16,
	pub __elision: i16,
	pub __list: __pthread_list_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s_anon_struct {
	pub __low: u32,
	pub __high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s_anon_union_a {
	pub __wseq: u64,
	pub __wseq32: __pthread_cond_s_anon_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s_anon_union_b {
	pub __g1_start: u64,
	pub __g1_start32: __pthread_cond_s_anon_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
	pub anon_a: __pthread_cond_s_anon_union_a,
	pub anon_b: __pthread_cond_s_anon_union_b,
	pub __g_refs: [u32; 2],
	pub __g_size: [u32; 2],
	pub __g1_orig_size: u32,
	pub __wrefs: u32,
	pub __g_signals: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __pthread_rwlock_arch_t {
	pub __readers: u32,
	pub __writers: u32,
	pub __wrphase_futex: u32,
	pub __writers_futex: u32,
	pub __pad3: u32,
	pub __pad4: u32,
	pub __cur_writer: i32,
	pub __shared: i32,
	pub __rwelision: i8,
	pub __pad1: [u8; 7],
	pub __pad2: u64,
	pub __flags: u32,
}
