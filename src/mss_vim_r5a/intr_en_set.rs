#[doc = "Register `INTR_EN_SET` reader"]
pub type R = crate::R<IntrEnSetSpec>;
#[doc = "Register `INTR_EN_SET` writer"]
pub type W = crate::W<IntrEnSetSpec>;
#[doc = "Field `MASK` reader - 31:0\\]
This field is used to enable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Set Enable"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - 31:0\\]
This field is used to enable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Set Enable"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to enable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Set Enable"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to enable the mask of events in Group M Each bit corresponds to event Q where Q = Mx32+Bit Read 0 Disabled Read 1 Enabled Write 0 No effect Write 1 Set Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<IntrEnSetSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Group M Interrupt Enabled Set Register (M is 0 to 7) h400 + M x h20 + h08\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEnSetSpec;
impl crate::RegisterSpec for IntrEnSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_en_set::R`](R) reader structure"]
impl crate::Readable for IntrEnSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_en_set::W`](W) writer structure"]
impl crate::Writable for IntrEnSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_EN_SET to value 0"]
impl crate::Resettable for IntrEnSetSpec {
    const RESET_VALUE: u32 = 0;
}
