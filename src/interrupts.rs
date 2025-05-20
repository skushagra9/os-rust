use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};
use lazy_static::lazy_static;
use crate::pic;
use crate::task::keyboard::add_scancode;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[pic::InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        // Temporarily comment out keyboard handler
        // idt[pic::InterruptIndex::Keyboard.as_usize()]
        //     .set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        pic::PICS.lock().notify_end_of_interrupt(pic::InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    
    // Notify end of interrupt before potentially problematic code
    unsafe {
        pic::PICS.lock().notify_end_of_interrupt(pic::InterruptIndex::Keyboard.as_u8());
    }
    
    // Handle scancode after EOI to prevent deadlocks
    add_scancode(scancode);
}