#[doc = "Register `IRQSTS` reader"]
pub type R = crate::R<IrqstsSpec>;
#[doc = "Register `IRQSTS` writer"]
pub type W = crate::W<IrqstsSpec>;
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
    pub fn mask(&mut self) -> MaskW<IrqstsSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Group M Interrupt IRQ Enabled Status/Clear Register (M is 0 to 7) h400 + M x h20 + h10\n\nYou can [`read`](crate::Reg::read) this register and get [`irqsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqstsSpec;
impl crate::RegisterSpec for IrqstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqsts::R`](R) reader structure"]
impl crate::Readable for IrqstsSpec {}
#[doc = "`write(|w| ..)` method takes [`irqsts::W`](W) writer structure"]
impl crate::Writable for IrqstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSTS to value 0"]
impl crate::Resettable for IrqstsSpec {
    const RESET_VALUE: u32 = 0;
}
