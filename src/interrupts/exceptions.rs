//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
use x86_64::structures::idt::{ExceptionStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame,
                                               _error_code: u64)
{
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: divide_by_zero\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn debug_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: debug\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: non_maskable_interrupt\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn overflow_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: overflow\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: bound_range_exceeded\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: invalid_opcode\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn device_not_available_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: device_not_available\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn invalid_tss_handler(stack_frame: &mut ExceptionStackFrame,
                                                  _error_code: u64)
{
    println!("EXCEPTION: invalid_tss\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn segment_not_present_handler(stack_frame: &mut ExceptionStackFrame,
                                                          _error_code: u64)
{
    println!("EXCEPTION: segment_not_present\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: &mut ExceptionStackFrame,
                                                          _error_code: u64)
{
    println!("EXCEPTION: stack_segment_fault\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: &mut ExceptionStackFrame,
                                                               _error_code: u64)
{
    println!("EXCEPTION: general_protection_fault\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut ExceptionStackFrame,
                                                 _error_code: PageFaultErrorCode)
{
    println!("EXCEPTION: page_fault\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: x87_floating_point\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn alignment_check_handler(stack_frame: &mut ExceptionStackFrame,
                                                      _error_code: u64)
{
    println!("EXCEPTION: alignment_check\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn machine_check_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: machine_check\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: simd_floating_point\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn virtualization_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: virtualization\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn security_exception_handler(stack_frame: &mut ExceptionStackFrame,
                                                         _error_code: u64)
{
    println!("EXCEPTION: security_exception\n{:#?}", stack_frame);
    loop {}
}