#[doc = "Register `ALE_POLICETBLCTL` reader"]
pub type R = crate::R<AlePolicetblctlSpec>;
#[doc = "Register `ALE_POLICETBLCTL` writer"]
pub type W = crate::W<AlePolicetblctlSpec>;
#[doc = "Field `POLICER_ENTRY_INDEX` reader - 1:0\\]
Policer Entry Index - This field specifies the policing/classifier entry to be read or written. When writing to this field without setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be loaded into the POLICECFG0-7 registers. When writing to this field with setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be updated from the POLICECFG0-7 registers."]
pub type PolicerEntryIndexR = crate::FieldReader;
#[doc = "Field `POLICER_ENTRY_INDEX` writer - 1:0\\]
Policer Entry Index - This field specifies the policing/classifier entry to be read or written. When writing to this field without setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be loaded into the POLICECFG0-7 registers. When writing to this field with setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be updated from the POLICECFG0-7 registers."]
pub type PolicerEntryIndexW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE_ENABLE_SETTING` reader - 31:31\\]
Write Enable - Setting this bit will write the POLICECFG0-7 to the ~ipol_tbl_idx selected policing/classifier entry. Clearing this bit will read the ~ipol_tbl_idx selected policing/classifier entry into the POLICECFG0-7 registers."]
pub type WriteEnableSettingR = crate::BitReader;
#[doc = "Field `WRITE_ENABLE_SETTING` writer - 31:31\\]
Write Enable - Setting this bit will write the POLICECFG0-7 to the ~ipol_tbl_idx selected policing/classifier entry. Clearing this bit will read the ~ipol_tbl_idx selected policing/classifier entry into the POLICECFG0-7 registers."]
pub type WriteEnableSettingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Policer Entry Index - This field specifies the policing/classifier entry to be read or written. When writing to this field without setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be loaded into the POLICECFG0-7 registers. When writing to this field with setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be updated from the POLICECFG0-7 registers."]
    #[inline(always)]
    pub fn policer_entry_index(&self) -> PolicerEntryIndexR {
        PolicerEntryIndexR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Write Enable - Setting this bit will write the POLICECFG0-7 to the ~ipol_tbl_idx selected policing/classifier entry. Clearing this bit will read the ~ipol_tbl_idx selected policing/classifier entry into the POLICECFG0-7 registers."]
    #[inline(always)]
    pub fn write_enable_setting(&self) -> WriteEnableSettingR {
        WriteEnableSettingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Policer Entry Index - This field specifies the policing/classifier entry to be read or written. When writing to this field without setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be loaded into the POLICECFG0-7 registers. When writing to this field with setting the ~iwrite_enable=1 will cause the selected policing/classifier entry to be updated from the POLICECFG0-7 registers."]
    #[inline(always)]
    #[must_use]
    pub fn policer_entry_index(&mut self) -> PolicerEntryIndexW<AlePolicetblctlSpec> {
        PolicerEntryIndexW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Write Enable - Setting this bit will write the POLICECFG0-7 to the ~ipol_tbl_idx selected policing/classifier entry. Clearing this bit will read the ~ipol_tbl_idx selected policing/classifier entry into the POLICECFG0-7 registers."]
    #[inline(always)]
    #[must_use]
    pub fn write_enable_setting(&mut self) -> WriteEnableSettingW<AlePolicetblctlSpec> {
        WriteEnableSettingW::new(self, 31)
    }
}
#[doc = "The Policing Table Control is used to read or write the selected policing/classifier entry. The selected policing/classifier entry is only read or written after this register is written based on the value of the ~iwrite_enable bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policetblctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policetblctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicetblctlSpec;
impl crate::RegisterSpec for AlePolicetblctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policetblctl::R`](R) reader structure"]
impl crate::Readable for AlePolicetblctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_policetblctl::W`](W) writer structure"]
impl crate::Writable for AlePolicetblctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICETBLCTL to value 0"]
impl crate::Resettable for AlePolicetblctlSpec {
    const RESET_VALUE: u32 = 0;
}
