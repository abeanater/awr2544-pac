#[doc = "Register `Fault Address` reader"]
pub type R = crate::R<FaultAddressSpec>;
#[doc = "Register `Fault Address` writer"]
pub type W = crate::W<FaultAddressSpec>;
#[doc = "Field `fault_addr` reader - 31:0\\]
Fault address."]
pub type FaultAddrR = crate::FieldReader<u32>;
#[doc = "Field `fault_addr` writer - 31:0\\]
Fault address."]
pub type FaultAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Fault address."]
    #[inline(always)]
    pub fn fault_addr(&self) -> FaultAddrR {
        FaultAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Fault address."]
    #[inline(always)]
    #[must_use]
    pub fn fault_addr(&mut self) -> FaultAddrW<FaultAddressSpec> {
        FaultAddrW::new(self, 0)
    }
}
#[doc = "Fault Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultAddressSpec;
impl crate::RegisterSpec for FaultAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_address::R`](R) reader structure"]
impl crate::Readable for FaultAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_address::W`](W) writer structure"]
impl crate::Writable for FaultAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Fault Address to value 0"]
impl crate::Resettable for FaultAddressSpec {
    const RESET_VALUE: u32 = 0;
}
