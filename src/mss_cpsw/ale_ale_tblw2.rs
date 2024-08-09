#[doc = "Register `ALE_ALE_TBLW2` reader"]
pub type R = crate::R<AleAleTblw2Spec>;
#[doc = "Register `ALE_ALE_TBLW2` writer"]
pub type W = crate::W<AleAleTblw2Spec>;
#[doc = "Field `TABLE_ENTRY_BITS` reader - 6:0\\]
Table Entry bits \\[71:64\\]"]
pub type TableEntryBitsR = crate::FieldReader;
#[doc = "Field `TABLE_ENTRY_BITS` writer - 6:0\\]
Table Entry bits \\[71:64\\]"]
pub type TableEntryBitsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Table Entry bits \\[71:64\\]"]
    #[inline(always)]
    pub fn table_entry_bits(&self) -> TableEntryBitsR {
        TableEntryBitsR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Table Entry bits \\[71:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn table_entry_bits(&mut self) -> TableEntryBitsW<AleAleTblw2Spec> {
        TableEntryBitsW::new(self, 0)
    }
}
#[doc = "The ALE Table Word 2 is the most significant word of an ALE table entry.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_tblw2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_tblw2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleTblw2Spec;
impl crate::RegisterSpec for AleAleTblw2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_tblw2::R`](R) reader structure"]
impl crate::Readable for AleAleTblw2Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_tblw2::W`](W) writer structure"]
impl crate::Writable for AleAleTblw2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_TBLW2 to value 0"]
impl crate::Resettable for AleAleTblw2Spec {
    const RESET_VALUE: u32 = 0;
}
