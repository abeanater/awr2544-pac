#[doc = "Register `ERR_PARITY_ATCM1` reader"]
pub type R = crate::R<ErrParityAtcm1Spec>;
#[doc = "Register `ERR_PARITY_ATCM1` writer"]
pub type W = crate::W<ErrParityAtcm1Spec>;
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
    pub fn addr(&mut self) -> AddrW<ErrParityAtcm1Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "ERR_PARITY_ATCM1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_parity_atcm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_parity_atcm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrParityAtcm1Spec;
impl crate::RegisterSpec for ErrParityAtcm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_parity_atcm1::R`](R) reader structure"]
impl crate::Readable for ErrParityAtcm1Spec {}
#[doc = "`write(|w| ..)` method takes [`err_parity_atcm1::W`](W) writer structure"]
impl crate::Writable for ErrParityAtcm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_PARITY_ATCM1 to value 0"]
impl crate::Resettable for ErrParityAtcm1Spec {
    const RESET_VALUE: u32 = 0;
}
