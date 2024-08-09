#[doc = "Register `Programmable 6 Start Address` reader"]
pub type R = crate::R<Programmable6startAddressSpec>;
#[doc = "Register `Programmable 6 Start Address` writer"]
pub type W = crate::W<Programmable6startAddressSpec>;
#[doc = "Field `start_addr` reader - 31:0\\]
Start address for range N. Defaults to input signal value."]
pub type StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `start_addr` writer - 31:0\\]
Start address for range N. Defaults to input signal value."]
pub type StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Start address for range N. Defaults to input signal value."]
    #[inline(always)]
    pub fn start_addr(&self) -> StartAddrR {
        StartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Start address for range N. Defaults to input signal value."]
    #[inline(always)]
    #[must_use]
    pub fn start_addr(&mut self) -> StartAddrW<Programmable6startAddressSpec> {
        StartAddrW::new(self, 0)
    }
}
#[doc = "Programmable 6 Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable6start_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable6start_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Programmable6startAddressSpec;
impl crate::RegisterSpec for Programmable6startAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`programmable6start_address::R`](R) reader structure"]
impl crate::Readable for Programmable6startAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`programmable6start_address::W`](W) writer structure"]
impl crate::Writable for Programmable6startAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Programmable 6 Start Address to value 0"]
impl crate::Resettable for Programmable6startAddressSpec {
    const RESET_VALUE: u32 = 0;
}
