#[doc = "Register `STC_CADDR2` reader"]
pub type R = crate::R<StcCaddr2Spec>;
#[doc = "Register `STC_CADDR2` writer"]
pub type W = crate::W<StcCaddr2Spec>;
#[doc = "Field `ADDR` reader - 31:0\\]
Current ROM Address for CORE2 This register reflects the current ROM address(for micro code load) accessed during selftest for CORE2 in of case segment0."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Current ROM Address for CORE2 This register reflects the current ROM address(for micro code load) accessed during selftest for CORE2 in of case segment0."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current ROM Address for CORE2 This register reflects the current ROM address(for micro code load) accessed during selftest for CORE2 in of case segment0."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current ROM Address for CORE2 This register reflects the current ROM address(for micro code load) accessed during selftest for CORE2 in of case segment0."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<StcCaddr2Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Current Address register for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_caddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_caddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcCaddr2Spec;
impl crate::RegisterSpec for StcCaddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stc_caddr2::R`](R) reader structure"]
impl crate::Readable for StcCaddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`stc_caddr2::W`](W) writer structure"]
impl crate::Writable for StcCaddr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STC_CADDR2 to value 0"]
impl crate::Resettable for StcCaddr2Spec {
    const RESET_VALUE: u32 = 0;
}
