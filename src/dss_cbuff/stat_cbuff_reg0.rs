#[doc = "Register `STAT_CBUFF_REG0` reader"]
pub type R = crate::R<StatCbuffReg0Spec>;
#[doc = "Register `STAT_CBUFF_REG0` writer"]
pub type W = crate::W<StatCbuffReg0Spec>;
#[doc = "Field `S_CSI_PKT_RCVD` reader - 0:0\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine"]
pub type SCsiPktRcvdR = crate::BitReader;
#[doc = "Field `S_CSI_PKT_RCVD` writer - 0:0\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine"]
pub type SCsiPktRcvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CSI_PKT_VS_RCVD_STATE` reader - 1:1\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync Start Packet"]
pub type SCsiPktVsRcvdStateR = crate::BitReader;
#[doc = "Field `S_CSI_PKT_VS_RCVD_STATE` writer - 1:1\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync Start Packet"]
pub type SCsiPktVsRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CSI_PKT_VE_RCVD_STATE` reader - 2:2\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync End Packet"]
pub type SCsiPktVeRcvdStateR = crate::BitReader;
#[doc = "Field `S_CSI_PKT_VE_RCVD_STATE` writer - 2:2\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync End Packet"]
pub type SCsiPktVeRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CSI_PKT_HS_RCVD_STATE` reader - 3:3\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync Start Packet"]
pub type SCsiPktHsRcvdStateR = crate::BitReader;
#[doc = "Field `S_CSI_PKT_HS_RCVD_STATE` writer - 3:3\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync Start Packet"]
pub type SCsiPktHsRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CSI_PKT_HE_RCVD_STATE` reader - 4:4\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync End Packet"]
pub type SCsiPktHeRcvdStateR = crate::BitReader;
#[doc = "Field `S_CSI_PKT_HE_RCVD_STATE` writer - 4:4\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync End Packet"]
pub type SCsiPktHeRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_CSI_PKT_LP_RCVD_STATE` reader - 5:5\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Long Data Packet"]
pub type SCsiPktLpRcvdStateR = crate::BitReader;
#[doc = "Field `S_CSI_PKT_LP_RCVD_STATE` writer - 5:5\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Long Data Packet"]
pub type SCsiPktLpRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_LL_INDEX` reader - 10:6\\]
TI Internal Feature. Debug only. Current Linked list index."]
pub type SLlIndexR = crate::FieldReader;
#[doc = "Field `S_LL_INDEX` writer - 10:6\\]
TI Internal Feature. Debug only. Current Linked list index."]
pub type SLlIndexW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `S_CHIRP_DONE` reader - 11:11\\]
Indicates that CBUFF has completed sending out data for the current Chirp"]
pub type SChirpDoneR = crate::BitReader;
#[doc = "Field `S_CHIRP_DONE` writer - 11:11\\]
Indicates that CBUFF has completed sending out data for the current Chirp"]
pub type SChirpDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_FRAME_DONE` reader - 12:12\\]
Indicates that CBUFF has completed sending out data for the current Frame"]
pub type SFrameDoneR = crate::BitReader;
#[doc = "Field `S_FRAME_DONE` writer - 12:12\\]
Indicates that CBUFF has completed sending out data for the current Frame"]
pub type SFrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAT_CBUFF_REG0_OTHERS` reader - 31:13\\]
Reseved for future enhancement"]
pub type StatCbuffReg0OthersR = crate::FieldReader<u32>;
#[doc = "Field `STAT_CBUFF_REG0_OTHERS` writer - 31:13\\]
Reseved for future enhancement"]
pub type StatCbuffReg0OthersW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine"]
    #[inline(always)]
    pub fn s_csi_pkt_rcvd(&self) -> SCsiPktRcvdR {
        SCsiPktRcvdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync Start Packet"]
    #[inline(always)]
    pub fn s_csi_pkt_vs_rcvd_state(&self) -> SCsiPktVsRcvdStateR {
        SCsiPktVsRcvdStateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync End Packet"]
    #[inline(always)]
    pub fn s_csi_pkt_ve_rcvd_state(&self) -> SCsiPktVeRcvdStateR {
        SCsiPktVeRcvdStateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync Start Packet"]
    #[inline(always)]
    pub fn s_csi_pkt_hs_rcvd_state(&self) -> SCsiPktHsRcvdStateR {
        SCsiPktHsRcvdStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync End Packet"]
    #[inline(always)]
    pub fn s_csi_pkt_he_rcvd_state(&self) -> SCsiPktHeRcvdStateR {
        SCsiPktHeRcvdStateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Long Data Packet"]
    #[inline(always)]
    pub fn s_csi_pkt_lp_rcvd_state(&self) -> SCsiPktLpRcvdStateR {
        SCsiPktLpRcvdStateR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - 10:6\\]
TI Internal Feature. Debug only. Current Linked list index."]
    #[inline(always)]
    pub fn s_ll_index(&self) -> SLlIndexR {
        SLlIndexR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates that CBUFF has completed sending out data for the current Chirp"]
    #[inline(always)]
    pub fn s_chirp_done(&self) -> SChirpDoneR {
        SChirpDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates that CBUFF has completed sending out data for the current Frame"]
    #[inline(always)]
    pub fn s_frame_done(&self) -> SFrameDoneR {
        SFrameDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Reseved for future enhancement"]
    #[inline(always)]
    pub fn stat_cbuff_reg0_others(&self) -> StatCbuffReg0OthersR {
        StatCbuffReg0OthersR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine"]
    #[inline(always)]
    #[must_use]
    pub fn s_csi_pkt_rcvd(&mut self) -> SCsiPktRcvdW<StatCbuffReg0Spec> {
        SCsiPktRcvdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync Start Packet"]
    #[inline(always)]
    #[must_use]
    pub fn s_csi_pkt_vs_rcvd_state(&mut self) -> SCsiPktVsRcvdStateW<StatCbuffReg0Spec> {
        SCsiPktVsRcvdStateW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Vsync End Packet"]
    #[inline(always)]
    #[must_use]
    pub fn s_csi_pkt_ve_rcvd_state(&mut self) -> SCsiPktVeRcvdStateW<StatCbuffReg0Spec> {
        SCsiPktVeRcvdStateW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync Start Packet"]
    #[inline(always)]
    #[must_use]
    pub fn s_csi_pkt_hs_rcvd_state(&mut self) -> SCsiPktHsRcvdStateW<StatCbuffReg0Spec> {
        SCsiPktHsRcvdStateW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Hsync End Packet"]
    #[inline(always)]
    #[must_use]
    pub fn s_csi_pkt_he_rcvd_state(&mut self) -> SCsiPktHeRcvdStateW<StatCbuffReg0Spec> {
        SCsiPktHeRcvdStateW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal Feature Indicates that the CSI-2 Packet Received is sent to the CBUFF from the Protocol Engine after Long Data Packet"]
    #[inline(always)]
    #[must_use]
    pub fn s_csi_pkt_lp_rcvd_state(&mut self) -> SCsiPktLpRcvdStateW<StatCbuffReg0Spec> {
        SCsiPktLpRcvdStateW::new(self, 5)
    }
    #[doc = "Bits 6:10 - 10:6\\]
TI Internal Feature. Debug only. Current Linked list index."]
    #[inline(always)]
    #[must_use]
    pub fn s_ll_index(&mut self) -> SLlIndexW<StatCbuffReg0Spec> {
        SLlIndexW::new(self, 6)
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates that CBUFF has completed sending out data for the current Chirp"]
    #[inline(always)]
    #[must_use]
    pub fn s_chirp_done(&mut self) -> SChirpDoneW<StatCbuffReg0Spec> {
        SChirpDoneW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates that CBUFF has completed sending out data for the current Frame"]
    #[inline(always)]
    #[must_use]
    pub fn s_frame_done(&mut self) -> SFrameDoneW<StatCbuffReg0Spec> {
        SFrameDoneW::new(self, 12)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Reseved for future enhancement"]
    #[inline(always)]
    #[must_use]
    pub fn stat_cbuff_reg0_others(&mut self) -> StatCbuffReg0OthersW<StatCbuffReg0Spec> {
        StatCbuffReg0OthersW::new(self, 13)
    }
}
#[doc = "STAT_CBUFF_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cbuff_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_cbuff_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatCbuffReg0Spec;
impl crate::RegisterSpec for StatCbuffReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_cbuff_reg0::R`](R) reader structure"]
impl crate::Readable for StatCbuffReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`stat_cbuff_reg0::W`](W) writer structure"]
impl crate::Writable for StatCbuffReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT_CBUFF_REG0 to value 0"]
impl crate::Resettable for StatCbuffReg0Spec {
    const RESET_VALUE: u32 = 0;
}
