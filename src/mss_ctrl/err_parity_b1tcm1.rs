#[doc = "Register `ERR_PARITY_B1TCM1` reader"]
pub type R = crate::R<ErrParityB1tcm1Spec>;
#[doc = "Register `ERR_PARITY_B1TCM1` writer"]
pub type W = crate::W<ErrParityB1tcm1Spec>;
#[doc = "Field `addr` reader - 19:0\\]
RESERVED: Dont Use"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `addr` writer - 19:0\\]
RESERVED: Dont Use"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<ErrParityB1tcm1Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "ERR_PARITY_B1TCM1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_b1tcm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_b1tcm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrParityB1tcm1Spec;
impl crate::RegisterSpec for ErrParityB1tcm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_parity_b1tcm1::R`](R) reader structure"]
impl crate::Readable for ErrParityB1tcm1Spec {}
#[doc = "`write(|w| ..)` method takes [`err_parity_b1tcm1::W`](W) writer structure"]
impl crate::Writable for ErrParityB1tcm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_PARITY_B1TCM1 to value 0"]
impl crate::Resettable for ErrParityB1tcm1Spec {
    const RESET_VALUE: u32 = 0;
}
