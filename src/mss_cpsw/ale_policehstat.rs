#[doc = "Register `ALE_POLICEHSTAT` reader"]
pub type R = crate::R<AlePolicehstatSpec>;
#[doc = "Register `ALE_POLICEHSTAT` writer"]
pub type W = crate::W<AlePolicehstatSpec>;
#[doc = "Field `POLICER_HIT_YELLOW` reader - 29:29\\]
Policer Hit YELLOW - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a YELLOW condition by a packet seen on any port that matches the policing/classifier entry match."]
pub type PolicerHitYellowR = crate::BitReader;
#[doc = "Field `POLICER_HIT_YELLOW` writer - 29:29\\]
Policer Hit YELLOW - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a YELLOW condition by a packet seen on any port that matches the policing/classifier entry match."]
pub type PolicerHitYellowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLICER_HIT_RED` reader - 30:30\\]
Policer Hit RED - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a RED condition by a packet seen on any port that matches the policing/classifier entry match."]
pub type PolicerHitRedR = crate::BitReader;
#[doc = "Field `POLICER_HIT_RED` writer - 30:30\\]
Policer Hit RED - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a RED condition by a packet seen on any port that matches the policing/classifier entry match."]
pub type PolicerHitRedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLICER_HIT_THIS` reader - 31:31\\]
Policer Hit - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit by a packet seen on any port that matches the policing/classifier entry match."]
pub type PolicerHitThisR = crate::BitReader;
#[doc = "Field `POLICER_HIT_THIS` writer - 31:31\\]
Policer Hit - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit by a packet seen on any port that matches the policing/classifier entry match."]
pub type PolicerHitThisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - 29:29\\]
Policer Hit YELLOW - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a YELLOW condition by a packet seen on any port that matches the policing/classifier entry match."]
    #[inline(always)]
    pub fn policer_hit_yellow(&self) -> PolicerHitYellowR {
        PolicerHitYellowR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Policer Hit RED - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a RED condition by a packet seen on any port that matches the policing/classifier entry match."]
    #[inline(always)]
    pub fn policer_hit_red(&self) -> PolicerHitRedR {
        PolicerHitRedR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Policer Hit - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit by a packet seen on any port that matches the policing/classifier entry match."]
    #[inline(always)]
    pub fn policer_hit_this(&self) -> PolicerHitThisR {
        PolicerHitThisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - 29:29\\]
Policer Hit YELLOW - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a YELLOW condition by a packet seen on any port that matches the policing/classifier entry match."]
    #[inline(always)]
    #[must_use]
    pub fn policer_hit_yellow(&mut self) -> PolicerHitYellowW<AlePolicehstatSpec> {
        PolicerHitYellowW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Policer Hit RED - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit during a RED condition by a packet seen on any port that matches the policing/classifier entry match."]
    #[inline(always)]
    #[must_use]
    pub fn policer_hit_red(&mut self) -> PolicerHitRedW<AlePolicehstatSpec> {
        PolicerHitRedW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Policer Hit - This indicates that the selected policing/classifier via the ~ipol_test_idx field has been hit by a packet seen on any port that matches the policing/classifier entry match."]
    #[inline(always)]
    #[must_use]
    pub fn policer_hit_this(&mut self) -> PolicerHitThisW<AlePolicehstatSpec> {
        PolicerHitThisW::new(self, 31)
    }
}
#[doc = "The policing hit status is a read only register that reads the hit bits of the selected policing/classifier.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policehstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policehstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicehstatSpec;
impl crate::RegisterSpec for AlePolicehstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policehstat::R`](R) reader structure"]
impl crate::Readable for AlePolicehstatSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_policehstat::W`](W) writer structure"]
impl crate::Writable for AlePolicehstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICEHSTAT to value 0"]
impl crate::Resettable for AlePolicehstatSpec {
    const RESET_VALUE: u32 = 0;
}
