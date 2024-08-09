#[doc = "Register `HWA_SAFETY_WINDOW_RAM_ERR_ADDR` reader"]
pub type R = crate::R<HwaSafetyWindowRamErrAddrSpec>;
#[doc = "Register `HWA_SAFETY_WINDOW_RAM_ERR_ADDR` writer"]
pub type W = crate::W<HwaSafetyWindowRamErrAddrSpec>;
#[doc = "Field `window_ram_err_addr` reader - 10:0\\]
Captures the address where parity error occured for window RAM"]
pub type WindowRamErrAddrR = crate::FieldReader<u16>;
#[doc = "Field `window_ram_err_addr` writer - 10:0\\]
Captures the address where parity error occured for window RAM"]
pub type WindowRamErrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Captures the address where parity error occured for window RAM"]
    #[inline(always)]
    pub fn window_ram_err_addr(&self) -> WindowRamErrAddrR {
        WindowRamErrAddrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Captures the address where parity error occured for window RAM"]
    #[inline(always)]
    #[must_use]
    pub fn window_ram_err_addr(&mut self) -> WindowRamErrAddrW<HwaSafetyWindowRamErrAddrSpec> {
        WindowRamErrAddrW::new(self, 0)
    }
}
#[doc = "HWA_SAFETY_WINDOW_RAM_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_window_ram_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_window_ram_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyWindowRamErrAddrSpec;
impl crate::RegisterSpec for HwaSafetyWindowRamErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_window_ram_err_addr::R`](R) reader structure"]
impl crate::Readable for HwaSafetyWindowRamErrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_window_ram_err_addr::W`](W) writer structure"]
impl crate::Writable for HwaSafetyWindowRamErrAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_WINDOW_RAM_ERR_ADDR to value 0"]
impl crate::Resettable for HwaSafetyWindowRamErrAddrSpec {
    const RESET_VALUE: u32 = 0;
}
