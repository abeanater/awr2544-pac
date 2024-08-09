#[doc = "Register `HWA_SAFETY_DMEM0_ERR_ADDR` reader"]
pub type R = crate::R<HwaSafetyDmem0ErrAddrSpec>;
#[doc = "Register `HWA_SAFETY_DMEM0_ERR_ADDR` writer"]
pub type W = crate::W<HwaSafetyDmem0ErrAddrSpec>;
#[doc = "Field `dmem0_err_addr` reader - 9:0\\]
Captures the address where parity error occured for dmem0"]
pub type Dmem0ErrAddrR = crate::FieldReader<u16>;
#[doc = "Field `dmem0_err_addr` writer - 9:0\\]
Captures the address where parity error occured for dmem0"]
pub type Dmem0ErrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Captures the address where parity error occured for dmem0"]
    #[inline(always)]
    pub fn dmem0_err_addr(&self) -> Dmem0ErrAddrR {
        Dmem0ErrAddrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Captures the address where parity error occured for dmem0"]
    #[inline(always)]
    #[must_use]
    pub fn dmem0_err_addr(&mut self) -> Dmem0ErrAddrW<HwaSafetyDmem0ErrAddrSpec> {
        Dmem0ErrAddrW::new(self, 0)
    }
}
#[doc = "HWA_SAFETY_DMEM0_ERR_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_dmem0_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_dmem0_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyDmem0ErrAddrSpec;
impl crate::RegisterSpec for HwaSafetyDmem0ErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_dmem0_err_addr::R`](R) reader structure"]
impl crate::Readable for HwaSafetyDmem0ErrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_dmem0_err_addr::W`](W) writer structure"]
impl crate::Writable for HwaSafetyDmem0ErrAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_DMEM0_ERR_ADDR to value 0"]
impl crate::Resettable for HwaSafetyDmem0ErrAddrSpec {
    const RESET_VALUE: u32 = 0;
}
