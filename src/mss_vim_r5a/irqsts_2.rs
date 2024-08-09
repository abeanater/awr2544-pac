#[doc = "Register `IRQSTS_2` reader"]
pub type R = crate::R<Irqsts2Spec>;
#[doc = "Register `IRQSTS_2` writer"]
pub type W = crate::W<Irqsts2Spec>;
#[doc = "Field `MASK` reader - 31:0\\]
This is the masked status of the events in group M that are mapped to IRQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an IRQ Read 1 Active/Pending, Enabled, and IRQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if IRQ)"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - 31:0\\]
This is the masked status of the events in group M that are mapped to IRQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an IRQ Read 1 Active/Pending, Enabled, and IRQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if IRQ)"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status of the events in group M that are mapped to IRQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an IRQ Read 1 Active/Pending, Enabled, and IRQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if IRQ)"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status of the events in group M that are mapped to IRQ Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Inactive, Disabled, or not an IRQ Read 1 Active/Pending, Enabled, and IRQ Write 0 No effect Write 1 Clear Interrupt Raw Status (if IRQ)"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<Irqsts2Spec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irqsts2Spec;
impl crate::RegisterSpec for Irqsts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqsts_2::R`](R) reader structure"]
impl crate::Readable for Irqsts2Spec {}
#[doc = "`write(|w| ..)` method takes [`irqsts_2::W`](W) writer structure"]
impl crate::Writable for Irqsts2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSTS_2 to value 0"]
impl crate::Resettable for Irqsts2Spec {
    const RESET_VALUE: u32 = 0;
}
