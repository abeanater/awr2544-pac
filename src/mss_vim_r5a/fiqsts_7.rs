#[doc = "Register `FIQSTS_7` reader"]
pub type R = crate::R<Fiqsts7Spec>;
#[doc = "Register `FIQSTS_7` writer"]
pub type W = crate::W<Fiqsts7Spec>;
#[doc = "Field `MASK` reader - 31:0\\]
This is the masked status of the events in group M that are mapped to FIQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an FIQ Read 1 Active/Pending, Enabled, and FIQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if FIQ)"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - 31:0\\]
This is the masked status of the events in group M that are mapped to FIQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an FIQ Read 1 Active/Pending, Enabled, and FIQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if FIQ)"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status of the events in group M that are mapped to FIQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an FIQ Read 1 Active/Pending, Enabled, and FIQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if FIQ)"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status of the events in group M that are mapped to FIQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an FIQ Read 1 Active/Pending, Enabled, and FIQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if FIQ)"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<Fiqsts7Spec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Group M Interrupt FIQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h14\n\nYou can [`read`](crate::Reg::read) this register and get [`fiqsts_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiqsts_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fiqsts7Spec;
impl crate::RegisterSpec for Fiqsts7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiqsts_7::R`](R) reader structure"]
impl crate::Readable for Fiqsts7Spec {}
#[doc = "`write(|w| ..)` method takes [`fiqsts_7::W`](W) writer structure"]
impl crate::Writable for Fiqsts7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIQSTS_7 to value 0"]
impl crate::Resettable for Fiqsts7Spec {
    const RESET_VALUE: u32 = 0;
}
