#[doc = "Register `ALE_POLICETESTCTL` reader"]
pub type R = crate::R<AlePolicetestctlSpec>;
#[doc = "Register `ALE_POLICETESTCTL` writer"]
pub type W = crate::W<AlePolicetestctlSpec>;
#[doc = "Field `POLICER_TEST_INDEX` reader - 1:0\\]
Policer Test Index - This field selects which policing/classifier hit bits will be read or written."]
pub type PolicerTestIndexR = crate::FieldReader;
#[doc = "Field `POLICER_TEST_INDEX` writer - 1:0\\]
Policer Test Index - This field selects which policing/classifier hit bits will be read or written."]
pub type PolicerTestIndexW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POLICE_CLEAR_SELECTED` reader - 28:28\\]
Police Clear Selected - This bit clears the selected policing/classifier hit, redhit and yellowhit bits. This bit is self clearing."]
pub type PoliceClearSelectedR = crate::BitReader;
#[doc = "Field `POLICE_CLEAR_SELECTED` writer - 28:28\\]
Police Clear Selected - This bit clears the selected policing/classifier hit, redhit and yellowhit bits. This bit is self clearing."]
pub type PoliceClearSelectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLICER_CLEAR_YELLOW` reader - 29:29\\]
Policer Clear YELLOW - This bit clears all the policing/classifier YELLOW hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a YELLOW condition."]
pub type PolicerClearYellowR = crate::BitReader;
#[doc = "Field `POLICER_CLEAR_YELLOW` writer - 29:29\\]
Policer Clear YELLOW - This bit clears all the policing/classifier YELLOW hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a YELLOW condition."]
pub type PolicerClearYellowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLICER_CLEAR_RED` reader - 30:30\\]
Policer Clear RED - This bit clears all the policing/classifier RED hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a RED condition."]
pub type PolicerClearRedR = crate::BitReader;
#[doc = "Field `POLICER_CLEAR_RED` writer - 30:30\\]
Policer Clear RED - This bit clears all the policing/classifier RED hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a RED condition."]
pub type PolicerClearRedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLICER_CLEAR_THIS` reader - 31:31\\]
Policer Clear - This bit clears all the policing/classifier hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit."]
pub type PolicerClearThisR = crate::BitReader;
#[doc = "Field `POLICER_CLEAR_THIS` writer - 31:31\\]
Policer Clear - This bit clears all the policing/classifier hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit."]
pub type PolicerClearThisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Policer Test Index - This field selects which policing/classifier hit bits will be read or written."]
    #[inline(always)]
    pub fn policer_test_index(&self) -> PolicerTestIndexR {
        PolicerTestIndexR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Police Clear Selected - This bit clears the selected policing/classifier hit, redhit and yellowhit bits. This bit is self clearing."]
    #[inline(always)]
    pub fn police_clear_selected(&self) -> PoliceClearSelectedR {
        PoliceClearSelectedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Policer Clear YELLOW - This bit clears all the policing/classifier YELLOW hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a YELLOW condition."]
    #[inline(always)]
    pub fn policer_clear_yellow(&self) -> PolicerClearYellowR {
        PolicerClearYellowR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Policer Clear RED - This bit clears all the policing/classifier RED hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a RED condition."]
    #[inline(always)]
    pub fn policer_clear_red(&self) -> PolicerClearRedR {
        PolicerClearRedR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Policer Clear - This bit clears all the policing/classifier hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit."]
    #[inline(always)]
    pub fn policer_clear_this(&self) -> PolicerClearThisR {
        PolicerClearThisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Policer Test Index - This field selects which policing/classifier hit bits will be read or written."]
    #[inline(always)]
    #[must_use]
    pub fn policer_test_index(&mut self) -> PolicerTestIndexW<AlePolicetestctlSpec> {
        PolicerTestIndexW::new(self, 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Police Clear Selected - This bit clears the selected policing/classifier hit, redhit and yellowhit bits. This bit is self clearing."]
    #[inline(always)]
    #[must_use]
    pub fn police_clear_selected(&mut self) -> PoliceClearSelectedW<AlePolicetestctlSpec> {
        PoliceClearSelectedW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Policer Clear YELLOW - This bit clears all the policing/classifier YELLOW hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a YELLOW condition."]
    #[inline(always)]
    #[must_use]
    pub fn policer_clear_yellow(&mut self) -> PolicerClearYellowW<AlePolicetestctlSpec> {
        PolicerClearYellowW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Policer Clear RED - This bit clears all the policing/classifier RED hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit during a RED condition."]
    #[inline(always)]
    #[must_use]
    pub fn policer_clear_red(&mut self) -> PolicerClearRedW<AlePolicetestctlSpec> {
        PolicerClearRedW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Policer Clear - This bit clears all the policing/classifier hit bits. This bit is self clearing. This can be used to test the fact that a policing/classifier entry has been hit."]
    #[inline(always)]
    #[must_use]
    pub fn policer_clear_this(&mut self) -> PolicerClearThisW<AlePolicetestctlSpec> {
        PolicerClearThisW::new(self, 31)
    }
}
#[doc = "The Policing Test Control enables the ability to determine which policing entry has been hit and whether they reported a red or yellow rate condition.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policetestctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policetestctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicetestctlSpec;
impl crate::RegisterSpec for AlePolicetestctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policetestctl::R`](R) reader structure"]
impl crate::Readable for AlePolicetestctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_policetestctl::W`](W) writer structure"]
impl crate::Writable for AlePolicetestctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICETESTCTL to value 0"]
impl crate::Resettable for AlePolicetestctlSpec {
    const RESET_VALUE: u32 = 0;
}
