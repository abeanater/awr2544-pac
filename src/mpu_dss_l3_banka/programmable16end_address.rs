#[doc = "Register `Programmable 16 End Address` reader"]
pub type R = crate::R<Programmable16endAddressSpec>;
#[doc = "Register `Programmable 16 End Address` writer"]
pub type W = crate::W<Programmable16endAddressSpec>;
#[doc = "Field `end_addr` reader - 31:0\\]
Reserved not used in Design"]
pub type EndAddrR = crate::FieldReader<u32>;
#[doc = "Field `end_addr` writer - 31:0\\]
Reserved not used in Design"]
pub type EndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved not used in Design"]
    #[inline(always)]
    pub fn end_addr(&self) -> EndAddrR {
        EndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved not used in Design"]
    #[inline(always)]
    #[must_use]
    pub fn end_addr(&mut self) -> EndAddrW<Programmable16endAddressSpec> {
        EndAddrW::new(self, 0)
    }
}
#[doc = "Programmable 16 End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`programmable16end_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`programmable16end_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Programmable16endAddressSpec;
impl crate::RegisterSpec for Programmable16endAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`programmable16end_address::R`](R) reader structure"]
impl crate::Readable for Programmable16endAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`programmable16end_address::W`](W) writer structure"]
impl crate::Writable for Programmable16endAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Programmable 16 End Address to value 0"]
impl crate::Resettable for Programmable16endAddressSpec {
    const RESET_VALUE: u32 = 0;
}
