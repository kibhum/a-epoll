
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLET: i32 = 1 << 31;
#[link(name = "c")]
extern "C" {
//     This method accepts one argument called size, but size is there only for historical reasons.
// The argument will be ignored but must have a value larger than 0.
  pub fn epoll_create(size: i32) -> i32;
//   close is the syscall we need to close the file descriptor we get when we create our epoll
// instance, so we release our resources properly.
  pub fn close(fd: i32) -> i32;
//   This is the call we use to register interest in events on a source. It supports three main operations:
// add, modify, or delete. The first argument, epfd, is the epoll file descriptor we want to perform
// operations on. The second argument, op, is the argument where we specify whether we want
// to perform an add, modify, or delete operation
// epoll_event is a little more complicated, so we’ll discuss it in more detail. It does two important
// things for us: first, the events field indicates what kind of events we want to be notified of
// and it can also modify the behavior of how and when we get notified. Second, the data field
// passes on a piece of data to the kernel that it will return to us when an event occurs. The latter
// is important since we need this data to identify exactly what event occurred since that’s the
// only information we’ll receive in return that can identify what source we got the notification
// for
  pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
//   epoll_wait is the call that will block the current thread and wait until one of two things
//   happens: we receive a notification that an event has occurred or it times out. epfd is the epoll
//   file descriptor identifying the queue we made with epoll_create. events is an array of
//   the same Event structure we used in epoll_ctl. The difference is that the events field
//   now gives us information about what event did occur, and importantly the data field contains
//   the same data that we passed in when we registered interest
// the data field lets us identify which file descriptor has data that’s ready to be
// read. The maxevents arguments tell the kernel how many events we have reserved space
// for in our array. Lastly, the timeout argument tells the kernel how long we will wait for
// events before it will wake us up again so we don’t potentially block forever

  pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
// What does #[repr(packed)] do?
// The #[repr(packed)] annotation is new to us. Usually, a struct will have padding either
// between fields or at the end of the struct. This happens even when we’ve specified #[repr(C)].
// The reason has to do with efficient access to the data stored in the struct by not having to make
// multiple fetches to get the data stored in a struct field. In the case of the Event struct, the
// usual padding would be adding 4 bytes of padding at the end of the events field. When the
// operating system expects a packed struct for Event, and we give it a padded one, it will write
// parts of event_data to the padding between the fields. When you try to read event_data
// later on, you’ll end up onl
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,
    // Token to identify event
    pub(crate) epoll_data: usize,
}
impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}