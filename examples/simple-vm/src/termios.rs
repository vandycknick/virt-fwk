use std::{io, mem, os::fd::AsRawFd};

// Support functions for converting libc return values to io errors {
trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

pub fn get_terminal_attr(fd: &impl AsRawFd) -> io::Result<libc::termios> {
    unsafe {
        let mut termios = mem::zeroed();
        cvt(libc::tcgetattr(fd.as_raw_fd(), &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(fd: &impl AsRawFd, termios: &libc::termios) -> io::Result<()> {
    cvt(unsafe { libc::tcsetattr(fd.as_raw_fd(), libc::TCSANOW, termios) }).and(Ok(()))
}

pub fn set_raw_mode(fd: &impl AsRawFd) -> io::Result<()> {
    let mut attr = get_terminal_attr(fd).unwrap();

    // Put stdin into raw mode, disabling local echo, input canonicalization,
    // and CR-NL mapping.
    attr.c_iflag &= !libc::ICRNL;
    attr.c_lflag &= !(libc::ICANON | libc::ECHO);

    attr.c_cc[libc::VMIN] = 1;
    attr.c_cc[libc::VTIME] = 0;

    set_terminal_attr(fd, &attr).unwrap();

    Ok(())
}
