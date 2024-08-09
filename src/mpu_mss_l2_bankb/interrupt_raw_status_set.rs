#[doc = "Register `Interrupt Raw Status/Set` reader"]
pub type R = crate::R<InterruptRawStatusSetSpec>;
#[doc = "Register `Interrupt Raw Status/Set` writer"]
pub type W = crate::W<InterruptRawStatusSetSpec>;
#[doc = "Field `prot_err` reader - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtErrR = crate::BitReader;
#[doc = "Field `prot_err` writer - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `addr_err` reader - 1:1\\]
Addressing violation error. Raw status is read.Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddrErrR = crate::BitReader;
#[doc = "Field `addr_err` writer - 1:1\\]
Addressing violation error. Raw status is read.Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddrErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err(&self) -> ProtErrR {
        ProtErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read.Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err(&self) -> AddrErrR {
        AddrErrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err(&mut self) -> ProtErrW<InterruptRawStatusSetSpec> {
        ProtErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read.Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err(&mut self) -> AddrErrW<InterruptRawStatusSetSpec> {
        AddrErrW::new(self, 1)
    }
}
#[doc = "Interrupt Raw Status/Set\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_raw_status_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_raw_status_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptRawStatusSetSpec;
impl crate::RegisterSpec for InterruptRawStatusSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_raw_status_set::R`](R) reader structure"]
impl crate::Readable for InterruptRawStatusSetSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_raw_status_set::W`](W) writer structure"]
impl crate::Writable for InterruptRawStatusSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Interrupt Raw Status/Set to value 0"]
impl crate::Resettable for InterruptRawStatusSetSpec {
    const RESET_VALUE: u32 = 0;
}
