#[doc = "Register `ALE_ALE_TBLW0` reader"]
pub type R = crate::R<AleAleTblw0Spec>;
#[doc = "Register `ALE_ALE_TBLW0` writer"]
pub type W = crate::W<AleAleTblw0Spec>;
#[doc = "Field `TABLE_ENTRY_BITS` reader - 31:0\\]
Table Entry bits \\[31:0\\]"]
pub type TableEntryBitsR = crate::FieldReader<u32>;
#[doc = "Field `TABLE_ENTRY_BITS` writer - 31:0\\]
Table Entry bits \\[31:0\\]"]
pub type TableEntryBitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Table Entry bits \\[31:0\\]"]
    #[inline(always)]
    pub fn table_entry_bits(&self) -> TableEntryBitsR {
        TableEntryBitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Table Entry bits \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn table_entry_bits(&mut self) -> TableEntryBitsW<AleAleTblw0Spec> {
        TableEntryBitsW::new(self, 0)
    }
}
#[doc = "The ALE Table Word 0 is the least significant word of an ALE table entry.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_tblw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_tblw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleTblw0Spec;
impl crate::RegisterSpec for AleAleTblw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_tblw0::R`](R) reader structure"]
impl crate::Readable for AleAleTblw0Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_tblw0::W`](W) writer structure"]
impl crate::Writable for AleAleTblw0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_TBLW0 to value 0"]
impl crate::Resettable for AleAleTblw0Spec {
    const RESET_VALUE: u32 = 0;
}
