#[doc = "Register `Programmable 4 End Address` reader"]
pub type R = crate::R<Programmable4endAddressSpec>;
#[doc = "Register `Programmable 4 End Address` writer"]
pub type W = crate::W<Programmable4endAddressSpec>;
#[doc = "Field `end_addr` reader - 31:0\\]
End address for range N. Defaults to input signal value."]
pub type EndAddrR = crate::FieldReader<u32>;
#[doc = "Field `end_addr` writer - 31:0\\]
End address for range N. Defaults to input signal value."]
pub type EndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
End address for range N. Defaults to input signal value."]
    #[inline(always)]
    pub fn end_addr(&self) -> EndAddrR {
        EndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
End address for range N. Defaults to input signal value."]
    #[inline(always)]
    #[must_use]
    pub fn end_addr(&mut self) -> EndAddrW<Programmable4endAddressSpec> {
        EndAddrW::new(self, 0)
    }
}
#[doc = "Programmable 4 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable4end_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable4end_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Programmable4endAddressSpec;
impl crate::RegisterSpec for Programmable4endAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`programmable4end_address::R`](R) reader structure"]
impl crate::Readable for Programmable4endAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`programmable4end_address::W`](W) writer structure"]
impl crate::Writable for Programmable4endAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Programmable 4 End Address to value 0"]
impl crate::Resettable for Programmable4endAddressSpec {
    const RESET_VALUE: u32 = 0;
}
