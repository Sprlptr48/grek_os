
#![no_std]
#![no_main]


#[no_mangle]
pub extern "C" fn _start()  -> !{
	loop{} //i�letim sistemi i�in ba�lang�� noktas�
}


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
	loop{}  // panik durumunda �a�r�l�r
}
