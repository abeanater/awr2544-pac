#[doc = "Register `CLR_CBUFF_REG0` reader"]
pub type R = crate::R<ClrCbuffReg0Spec>;
#[doc = "Register `CLR_CBUFF_REG0` writer"]
pub type W = crate::W<ClrCbuffReg0Spec>;
#[doc = "Field `C_CSI_PKT_RCVD` reader - 0:0\\]
TI Internal Feature. Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktRcvdR = crate::BitReader;
#[doc = "Field `C_CSI_PKT_RCVD` writer - 0:0\\]
TI Internal Feature. Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktRcvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_CSI_PKT_VS_RCVD_STATE` reader - 1:1\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktVsRcvdStateR = crate::BitReader;
#[doc = "Field `C_CSI_PKT_VS_RCVD_STATE` writer - 1:1\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktVsRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_CSI_PKT_VE_RCVD_STATE` reader - 2:2\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktVeRcvdStateR = crate::BitReader;
#[doc = "Field `C_CSI_PKT_VE_RCVD_STATE` writer - 2:2\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktVeRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_CSI_PKT_HS_RCVD_STATE` reader - 3:3\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktHsRcvdStateR = crate::BitReader;
#[doc = "Field `C_CSI_PKT_HS_RCVD_STATE` writer - 3:3\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktHsRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_CSI_PKT_HE_RCVD_STATE` reader - 4:4\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktHeRcvdStateR = crate::BitReader;
#[doc = "Field `C_CSI_PKT_HE_RCVD_STATE` writer - 4:4\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktHeRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_CSI_PKT_LP_RCVD_STATE` reader - 5:5\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktLpRcvdStateR = crate::BitReader;
#[doc = "Field `C_CSI_PKT_LP_RCVD_STATE` writer - 5:5\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CCsiPktLpRcvdStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_LL_INDEX` reader - 10:6\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CLlIndexR = crate::FieldReader;
#[doc = "Field `C_LL_INDEX` writer - 10:6\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CLlIndexW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `C_CHIRP_DONE` reader - 11:11\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CChirpDoneR = crate::BitReader;
#[doc = "Field `C_CHIRP_DONE` writer - 11:11\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CChirpDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_FRAME_DONE` reader - 12:12\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CFrameDoneR = crate::BitReader;
#[doc = "Field `C_FRAME_DONE` writer - 12:12\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type CFrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_CBUFF_REG0_OTHERS` reader - 31:13\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type ClrCbuffReg0OthersR = crate::FieldReader<u32>;
#[doc = "Field `CLR_CBUFF_REG0_OTHERS` writer - 31:13\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
pub type ClrCbuffReg0OthersW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature. Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_csi_pkt_rcvd(&self) -> CCsiPktRcvdR {
        CCsiPktRcvdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_csi_pkt_vs_rcvd_state(&self) -> CCsiPktVsRcvdStateR {
        CCsiPktVsRcvdStateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_csi_pkt_ve_rcvd_state(&self) -> CCsiPktVeRcvdStateR {
        CCsiPktVeRcvdStateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_csi_pkt_hs_rcvd_state(&self) -> CCsiPktHsRcvdStateR {
        CCsiPktHsRcvdStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_csi_pkt_he_rcvd_state(&self) -> CCsiPktHeRcvdStateR {
        CCsiPktHeRcvdStateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_csi_pkt_lp_rcvd_state(&self) -> CCsiPktLpRcvdStateR {
        CCsiPktLpRcvdStateR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - 10:6\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_ll_index(&self) -> CLlIndexR {
        CLlIndexR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_chirp_done(&self) -> CChirpDoneR {
        CChirpDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn c_frame_done(&self) -> CFrameDoneR {
        CFrameDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    pub fn clr_cbuff_reg0_others(&self) -> ClrCbuffReg0OthersR {
        ClrCbuffReg0OthersR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature. Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_csi_pkt_rcvd(&mut self) -> CCsiPktRcvdW<ClrCbuffReg0Spec> {
        CCsiPktRcvdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_csi_pkt_vs_rcvd_state(&mut self) -> CCsiPktVsRcvdStateW<ClrCbuffReg0Spec> {
        CCsiPktVsRcvdStateW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_csi_pkt_ve_rcvd_state(&mut self) -> CCsiPktVeRcvdStateW<ClrCbuffReg0Spec> {
        CCsiPktVeRcvdStateW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_csi_pkt_hs_rcvd_state(&mut self) -> CCsiPktHsRcvdStateW<ClrCbuffReg0Spec> {
        CCsiPktHsRcvdStateW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_csi_pkt_he_rcvd_state(&mut self) -> CCsiPktHeRcvdStateW<ClrCbuffReg0Spec> {
        CCsiPktHeRcvdStateW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_csi_pkt_lp_rcvd_state(&mut self) -> CCsiPktLpRcvdStateW<ClrCbuffReg0Spec> {
        CCsiPktLpRcvdStateW::new(self, 5)
    }
    #[doc = "Bits 6:10 - 10:6\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_ll_index(&mut self) -> CLlIndexW<ClrCbuffReg0Spec> {
        CLlIndexW::new(self, 6)
    }
    #[doc = "Bit 11 - 11:11\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_chirp_done(&mut self) -> CChirpDoneW<ClrCbuffReg0Spec> {
        CChirpDoneW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn c_frame_done(&mut self) -> CFrameDoneW<ClrCbuffReg0Spec> {
        CFrameDoneW::new(self, 12)
    }
    #[doc = "Bits 13:31 - 31:13\\]
TI Internal Feature.Clear Register field corresponding to STAT_CBUFF_REG0. Write 0x1 to Clear the field"]
    #[inline(always)]
    #[must_use]
    pub fn clr_cbuff_reg0_others(&mut self) -> ClrCbuffReg0OthersW<ClrCbuffReg0Spec> {
        ClrCbuffReg0OthersW::new(self, 13)
    }
}
#[doc = "CLR_CBUFF_REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_cbuff_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_cbuff_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrCbuffReg0Spec;
impl crate::RegisterSpec for ClrCbuffReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_cbuff_reg0::R`](R) reader structure"]
impl crate::Readable for ClrCbuffReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`clr_cbuff_reg0::W`](W) writer structure"]
impl crate::Writable for ClrCbuffReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_CBUFF_REG0 to value 0"]
impl crate::Resettable for ClrCbuffReg0Spec {
    const RESET_VALUE: u32 = 0;
}
