#[doc = "Register `ALE_ALE_TBLCTL` reader"]
pub type R = crate::R<AleAleTblctlSpec>;
#[doc = "Register `ALE_ALE_TBLCTL` writer"]
pub type W = crate::W<AleAleTblctlSpec>;
#[doc = "Field `THE_TABLE_INDEX` reader - 4:0\\]
The table index is used to determine which lookup table entry is read or written."]
pub type TheTableIndexR = crate::FieldReader;
#[doc = "Field `THE_TABLE_INDEX` writer - 4:0\\]
The table index is used to determine which lookup table entry is read or written."]
pub type TheTableIndexW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TABLE_WRITE_THIS` reader - 31:31\\]
Table Write - This bit is used to write the table words to the lookup table.#br# 0 - Table Read Operation is performed. The contents of the ~b TABLEIDX entry will be read into the ~b ALE_TBLWx registers#br# 1 - Table write operation is performed. This will take the current contents from the ~b ALE_TBLWx registers and write them to the table at the specified ~b TABLEIDX."]
pub type TableWriteThisR = crate::BitReader;
#[doc = "Field `TABLE_WRITE_THIS` writer - 31:31\\]
Table Write - This bit is used to write the table words to the lookup table.#br# 0 - Table Read Operation is performed. The contents of the ~b TABLEIDX entry will be read into the ~b ALE_TBLWx registers#br# 1 - Table write operation is performed. This will take the current contents from the ~b ALE_TBLWx registers and write them to the table at the specified ~b TABLEIDX."]
pub type TableWriteThisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
The table index is used to determine which lookup table entry is read or written."]
    #[inline(always)]
    pub fn the_table_index(&self) -> TheTableIndexR {
        TheTableIndexR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Table Write - This bit is used to write the table words to the lookup table.#br# 0 - Table Read Operation is performed. The contents of the ~b TABLEIDX entry will be read into the ~b ALE_TBLWx registers#br# 1 - Table write operation is performed. This will take the current contents from the ~b ALE_TBLWx registers and write them to the table at the specified ~b TABLEIDX."]
    #[inline(always)]
    pub fn table_write_this(&self) -> TableWriteThisR {
        TableWriteThisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
The table index is used to determine which lookup table entry is read or written."]
    #[inline(always)]
    #[must_use]
    pub fn the_table_index(&mut self) -> TheTableIndexW<AleAleTblctlSpec> {
        TheTableIndexW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Table Write - This bit is used to write the table words to the lookup table.#br# 0 - Table Read Operation is performed. The contents of the ~b TABLEIDX entry will be read into the ~b ALE_TBLWx registers#br# 1 - Table write operation is performed. This will take the current contents from the ~b ALE_TBLWx registers and write them to the table at the specified ~b TABLEIDX."]
    #[inline(always)]
    #[must_use]
    pub fn table_write_this(&mut self) -> TableWriteThisW<AleAleTblctlSpec> {
        TableWriteThisW::new(self, 31)
    }
}
#[doc = "The ALE table control register is used to read or write that ALE table entries. After writing to this register any read or write to any ALE register will be stalled until the read or write operation completes.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_tblctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_tblctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleTblctlSpec;
impl crate::RegisterSpec for AleAleTblctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_tblctl::R`](R) reader structure"]
impl crate::Readable for AleAleTblctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_tblctl::W`](W) writer structure"]
impl crate::Writable for AleAleTblctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_TBLCTL to value 0"]
impl crate::Resettable for AleAleTblctlSpec {
    const RESET_VALUE: u32 = 0;
}
