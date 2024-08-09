#[doc = "Register `STC_CADDR` reader"]
pub type R = crate::R<StcCaddrSpec>;
#[doc = "Register `STC_CADDR` writer"]
pub type W = crate::W<StcCaddrSpec>;
#[doc = "Field `ADDR` reader - 31:0\\]
Current ROM Address for CORE1 This register reflects the current ROM address (for micro code load) accessed during selftest for CORE1 in of case segment0 and all the remaining segmentsn where n = 1 to 3)."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Current ROM Address for CORE1 This register reflects the current ROM address (for micro code load) accessed during selftest for CORE1 in of case segment0 and all the remaining segmentsn where n = 1 to 3)."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current ROM Address for CORE1 This register reflects the current ROM address (for micro code load) accessed during selftest for CORE1 in of case segment0 and all the remaining segmentsn where n = 1 to 3)."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current ROM Address for CORE1 This register reflects the current ROM address (for micro code load) accessed during selftest for CORE1 in of case segment0 and all the remaining segmentsn where n = 1 to 3)."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<StcCaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Current Address register for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_caddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_caddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcCaddrSpec;
impl crate::RegisterSpec for StcCaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stc_caddr::R`](R) reader structure"]
impl crate::Readable for StcCaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`stc_caddr::W`](W) writer structure"]
impl crate::Writable for StcCaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STC_CADDR to value 0"]
impl crate::Resettable for StcCaddrSpec {
    const RESET_VALUE: u32 = 0;
}
