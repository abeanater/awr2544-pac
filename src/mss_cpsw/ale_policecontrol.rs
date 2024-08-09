#[doc = "Register `ALE_POLICECONTROL` reader"]
pub type R = crate::R<AlePolicecontrolSpec>;
#[doc = "Register `ALE_POLICECONTROL` writer"]
pub type W = crate::W<AlePolicecontrolSpec>;
#[doc = "Field `MAC_ONLY_DEFAULT` reader - 20:20\\]
MAC Only Default Disable - This field when set disables the default thread on MAC Only Ports. That is the default thread will be {port,priority}. If the traffic matches a classifier with a thread mapping, the classifier thread mapping still occurs."]
pub type MacOnlyDefaultR = crate::BitReader;
#[doc = "Field `MAC_ONLY_DEFAULT` writer - 20:20\\]
MAC Only Default Disable - This field when set disables the default thread on MAC Only Ports. That is the default thread will be {port,priority}. If the traffic matches a classifier with a thread mapping, the classifier thread mapping still occurs."]
pub type MacOnlyDefaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIORITY_THREAD_ENABLE` reader - 21:21\\]
Priority Thread Enable - This field determines if priority is OR'd to the default thread when no classifiers hit and the default thread is enabled."]
pub type PriorityThreadEnableR = crate::BitReader;
#[doc = "Field `PRIORITY_THREAD_ENABLE` writer - 21:21\\]
Priority Thread Enable - This field determines if priority is OR'd to the default thread when no classifiers hit and the default thread is enabled."]
pub type PriorityThreadEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLICING_MATCH_MODE` reader - 23:22\\]
Policing Match Mode - This field determines what happens to packets that fail to hit any policing/classifier entry.#br# 0 - No Hit packets are marked GREEN#br# 1 - No Hit packets are marked YELLOW#br# 2 - No Hit packets are marked RED#br# 3 - No Hit packets are marked based on policing/classifier entry=0 state."]
pub type PolicingMatchModeR = crate::FieldReader;
#[doc = "Field `POLICING_MATCH_MODE` writer - 23:22\\]
Policing Match Mode - This field determines what happens to packets that fail to hit any policing/classifier entry.#br# 0 - No Hit packets are marked GREEN#br# 1 - No Hit packets are marked YELLOW#br# 2 - No Hit packets are marked RED#br# 3 - No Hit packets are marked based on policing/classifier entry=0 state."]
pub type PolicingMatchModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `YELLOW_THRESHOLD_WHEN` reader - 26:24\\]
Yellow Threshold - When set enables a portion of the yellow packets to be dropped based on the ~iyellow_drop_en enable.#br# 0-100%#br# 1=50%#br# 2-33%#br# 3-25%#br# 4=20%#br# 5-17%#br# 6-14%#br# 7-13%"]
pub type YellowThresholdWhenR = crate::FieldReader;
#[doc = "Field `YELLOW_THRESHOLD_WHEN` writer - 26:24\\]
Yellow Threshold - When set enables a portion of the yellow packets to be dropped based on the ~iyellow_drop_en enable.#br# 0-100%#br# 1=50%#br# 2-33%#br# 3-25%#br# 4=20%#br# 5-17%#br# 6-14%#br# 7-13%"]
pub type YellowThresholdWhenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WELLOW_DROP_ENABLE` reader - 28:28\\]
WELLOW Drop Enable - Enables the ALE to drop yellow packets based on the ~iyellowthresh value. This field would normally not be used as to let the switch drop packets at a buffer threshold instead. In the event that the switch does not enable buffer threshold dropping, YELLOW packets can be dropped based on this feature."]
pub type WellowDropEnableR = crate::BitReader;
#[doc = "Field `WELLOW_DROP_ENABLE` writer - 28:28\\]
WELLOW Drop Enable - Enables the ALE to drop yellow packets based on the ~iyellowthresh value. This field would normally not be used as to let the switch drop packets at a buffer threshold instead. In the event that the switch does not enable buffer threshold dropping, YELLOW packets can be dropped based on this feature."]
pub type WellowDropEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_DROP_ENABLE` reader - 29:29\\]
RED Drop Enable - Enables the ALE to drop the red colored packets."]
pub type RedDropEnableR = crate::BitReader;
#[doc = "Field `RED_DROP_ENABLE` writer - 29:29\\]
RED Drop Enable - Enables the ALE to drop the red colored packets."]
pub type RedDropEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLICING_ENABLE_ENABLES` reader - 31:31\\]
Policing Enable - Enables the policing to color the packets, this also enables red or yellow drop capabilities."]
pub type PolicingEnableEnablesR = crate::BitReader;
#[doc = "Field `POLICING_ENABLE_ENABLES` writer - 31:31\\]
Policing Enable - Enables the policing to color the packets, this also enables red or yellow drop capabilities."]
pub type PolicingEnableEnablesW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20 - 20:20\\]
MAC Only Default Disable - This field when set disables the default thread on MAC Only Ports. That is the default thread will be {port,priority}. If the traffic matches a classifier with a thread mapping, the classifier thread mapping still occurs."]
    #[inline(always)]
    pub fn mac_only_default(&self) -> MacOnlyDefaultR {
        MacOnlyDefaultR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Priority Thread Enable - This field determines if priority is OR'd to the default thread when no classifiers hit and the default thread is enabled."]
    #[inline(always)]
    pub fn priority_thread_enable(&self) -> PriorityThreadEnableR {
        PriorityThreadEnableR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Policing Match Mode - This field determines what happens to packets that fail to hit any policing/classifier entry.#br# 0 - No Hit packets are marked GREEN#br# 1 - No Hit packets are marked YELLOW#br# 2 - No Hit packets are marked RED#br# 3 - No Hit packets are marked based on policing/classifier entry=0 state."]
    #[inline(always)]
    pub fn policing_match_mode(&self) -> PolicingMatchModeR {
        PolicingMatchModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Yellow Threshold - When set enables a portion of the yellow packets to be dropped based on the ~iyellow_drop_en enable.#br# 0-100%#br# 1=50%#br# 2-33%#br# 3-25%#br# 4=20%#br# 5-17%#br# 6-14%#br# 7-13%"]
    #[inline(always)]
    pub fn yellow_threshold_when(&self) -> YellowThresholdWhenR {
        YellowThresholdWhenR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
WELLOW Drop Enable - Enables the ALE to drop yellow packets based on the ~iyellowthresh value. This field would normally not be used as to let the switch drop packets at a buffer threshold instead. In the event that the switch does not enable buffer threshold dropping, YELLOW packets can be dropped based on this feature."]
    #[inline(always)]
    pub fn wellow_drop_enable(&self) -> WellowDropEnableR {
        WellowDropEnableR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
RED Drop Enable - Enables the ALE to drop the red colored packets."]
    #[inline(always)]
    pub fn red_drop_enable(&self) -> RedDropEnableR {
        RedDropEnableR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Policing Enable - Enables the policing to color the packets, this also enables red or yellow drop capabilities."]
    #[inline(always)]
    pub fn policing_enable_enables(&self) -> PolicingEnableEnablesR {
        PolicingEnableEnablesR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - 20:20\\]
MAC Only Default Disable - This field when set disables the default thread on MAC Only Ports. That is the default thread will be {port,priority}. If the traffic matches a classifier with a thread mapping, the classifier thread mapping still occurs."]
    #[inline(always)]
    #[must_use]
    pub fn mac_only_default(&mut self) -> MacOnlyDefaultW<AlePolicecontrolSpec> {
        MacOnlyDefaultW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Priority Thread Enable - This field determines if priority is OR'd to the default thread when no classifiers hit and the default thread is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn priority_thread_enable(&mut self) -> PriorityThreadEnableW<AlePolicecontrolSpec> {
        PriorityThreadEnableW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Policing Match Mode - This field determines what happens to packets that fail to hit any policing/classifier entry.#br# 0 - No Hit packets are marked GREEN#br# 1 - No Hit packets are marked YELLOW#br# 2 - No Hit packets are marked RED#br# 3 - No Hit packets are marked based on policing/classifier entry=0 state."]
    #[inline(always)]
    #[must_use]
    pub fn policing_match_mode(&mut self) -> PolicingMatchModeW<AlePolicecontrolSpec> {
        PolicingMatchModeW::new(self, 22)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Yellow Threshold - When set enables a portion of the yellow packets to be dropped based on the ~iyellow_drop_en enable.#br# 0-100%#br# 1=50%#br# 2-33%#br# 3-25%#br# 4=20%#br# 5-17%#br# 6-14%#br# 7-13%"]
    #[inline(always)]
    #[must_use]
    pub fn yellow_threshold_when(&mut self) -> YellowThresholdWhenW<AlePolicecontrolSpec> {
        YellowThresholdWhenW::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
WELLOW Drop Enable - Enables the ALE to drop yellow packets based on the ~iyellowthresh value. This field would normally not be used as to let the switch drop packets at a buffer threshold instead. In the event that the switch does not enable buffer threshold dropping, YELLOW packets can be dropped based on this feature."]
    #[inline(always)]
    #[must_use]
    pub fn wellow_drop_enable(&mut self) -> WellowDropEnableW<AlePolicecontrolSpec> {
        WellowDropEnableW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
RED Drop Enable - Enables the ALE to drop the red colored packets."]
    #[inline(always)]
    #[must_use]
    pub fn red_drop_enable(&mut self) -> RedDropEnableW<AlePolicecontrolSpec> {
        RedDropEnableW::new(self, 29)
    }
    #[doc = "Bit 31 - 31:31\\]
Policing Enable - Enables the policing to color the packets, this also enables red or yellow drop capabilities."]
    #[inline(always)]
    #[must_use]
    pub fn policing_enable_enables(&mut self) -> PolicingEnableEnablesW<AlePolicecontrolSpec> {
        PolicingEnableEnablesW::new(self, 31)
    }
}
#[doc = "The Control Enables color marking as well as internal ALE packet dropping rules.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecontrolSpec;
impl crate::RegisterSpec for AlePolicecontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecontrol::R`](R) reader structure"]
impl crate::Readable for AlePolicecontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecontrol::W`](W) writer structure"]
impl crate::Writable for AlePolicecontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECONTROL to value 0"]
impl crate::Resettable for AlePolicecontrolSpec {
    const RESET_VALUE: u32 = 0;
}
