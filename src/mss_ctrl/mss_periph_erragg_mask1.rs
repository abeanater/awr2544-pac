#[doc = "Register `MSS_PERIPH_ERRAGG_MASK1` reader"]
pub type R = crate::R<MssPeriphErraggMask1Spec>;
#[doc = "Register `MSS_PERIPH_ERRAGG_MASK1` writer"]
pub type W = crate::W<MssPeriphErraggMask1Spec>;
#[doc = "Field `mpu_rd_mss_l2_banka` reader - 0:0\\]
Mask Interrupt from MPU_MSS_L2_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssL2BankaR = crate::BitReader;
#[doc = "Field `mpu_rd_mss_l2_banka` writer - 0:0\\]
Mask Interrupt from MPU_MSS_L2_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssL2BankaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_mss_l2_bankb` reader - 1:1\\]
Mask Interrupt from MPU_MSS_L2_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssL2BankbR = crate::BitReader;
#[doc = "Field `mpu_rd_mss_l2_bankb` writer - 1:1\\]
Mask Interrupt from MPU_MSS_L2_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssL2BankbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_hsm_dthe` reader - 2:2\\]
Mask Interrupt from MPU_HSM_DTHE to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdHsmDtheR = crate::BitReader;
#[doc = "Field `mpu_rd_hsm_dthe` writer - 2:2\\]
Mask Interrupt from MPU_HSM_DTHE to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdHsmDtheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_mss_mbox` reader - 3:3\\]
Mask Interrupt from MPU_MSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssMboxR = crate::BitReader;
#[doc = "Field `mpu_rd_mss_mbox` writer - 3:3\\]
Mask Interrupt from MPU_MSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssMboxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_mss_pcra` reader - 4:4\\]
Mask Interrupt from MPU_MSS_PCRA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssPcraR = crate::BitReader;
#[doc = "Field `mpu_rd_mss_pcra` writer - 4:4\\]
Mask Interrupt from MPU_MSS_PCRA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssPcraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_mss_qspi` reader - 5:5\\]
Mask Interrupt from MPU_MSS_QSPI to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssQspiR = crate::BitReader;
#[doc = "Field `mpu_rd_mss_qspi` writer - 5:5\\]
Mask Interrupt from MPU_MSS_QSPI to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssQspiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_mss_cr5a_axis` reader - 6:6\\]
Mask Interrupt from MPU_MSS_CR5A_AXIS to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssCr5aAxisR = crate::BitReader;
#[doc = "Field `mpu_rd_mss_cr5a_axis` writer - 6:6\\]
Mask Interrupt from MPU_MSS_CR5A_AXIS to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssCr5aAxisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_mss_cr5b_axis` reader - 7:7\\]
Mask Interrupt from MPU_MSS_L2_BANKC to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssCr5bAxisR = crate::BitReader;
#[doc = "Field `mpu_rd_mss_cr5b_axis` writer - 7:7\\]
Mask Interrupt from MPU_MSS_L2_BANKC to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdMssCr5bAxisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_l3_banka` reader - 8:8\\]
Mask Interrupt from MPU_DSS_L3_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdDssL3BankaR = crate::BitReader;
#[doc = "Field `mpu_rd_dss_l3_banka` writer - 8:8\\]
Mask Interrupt from MPU_DSS_L3_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdDssL3BankaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_l3_bankb` reader - 9:9\\]
Mask Interrupt from MPU_DSS_L3_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdDssL3BankbR = crate::BitReader;
#[doc = "Field `mpu_rd_dss_l3_bankb` writer - 9:9\\]
Mask Interrupt from MPU_DSS_L3_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdDssL3BankbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_l3_bankc` reader - 10:10\\]
RESERVED:Dont Use"]
pub type MpuRdDssL3BankcR = crate::BitReader;
#[doc = "Field `mpu_rd_dss_l3_bankc` writer - 10:10\\]
RESERVED:Dont Use"]
pub type MpuRdDssL3BankcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_l3_bankd` reader - 11:11\\]
RESERVED:Dont Use"]
pub type MpuRdDssL3BankdR = crate::BitReader;
#[doc = "Field `mpu_rd_dss_l3_bankd` writer - 11:11\\]
RESERVED:Dont Use"]
pub type MpuRdDssL3BankdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_hwa_dma0` reader - 12:12\\]
RESERVED:Dont Use"]
pub type MpuRdDssHwaDma0R = crate::BitReader;
#[doc = "Field `mpu_rd_dss_hwa_dma0` writer - 12:12\\]
RESERVED:Dont Use"]
pub type MpuRdDssHwaDma0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_hwa_dma1` reader - 13:13\\]
MRESERVED:Dont Use"]
pub type MpuRdDssHwaDma1R = crate::BitReader;
#[doc = "Field `mpu_rd_dss_hwa_dma1` writer - 13:13\\]
MRESERVED:Dont Use"]
pub type MpuRdDssHwaDma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_hwa_proc` reader - 14:14\\]
RESERVED:Dont Use"]
pub type MpuRdDssHwaProcR = crate::BitReader;
#[doc = "Field `mpu_rd_dss_hwa_proc` writer - 14:14\\]
RESERVED:Dont Use"]
pub type MpuRdDssHwaProcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_dss_mbox` reader - 15:15\\]
Mask Interrupt from MPU_DSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdDssMboxR = crate::BitReader;
#[doc = "Field `mpu_rd_dss_mbox` writer - 15:15\\]
Mask Interrupt from MPU_DSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdDssMboxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mpu_rd_hsm` reader - 16:16\\]
Mask Interrupt from MPU_DSS_HSM to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdHsmR = crate::BitReader;
#[doc = "Field `mpu_rd_hsm` writer - 16:16\\]
Mask Interrupt from MPU_DSS_HSM to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type MpuRdHsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Mask Interrupt from MPU_MSS_L2_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_mss_l2_banka(&self) -> MpuRdMssL2BankaR {
        MpuRdMssL2BankaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Mask Interrupt from MPU_MSS_L2_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_mss_l2_bankb(&self) -> MpuRdMssL2BankbR {
        MpuRdMssL2BankbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Mask Interrupt from MPU_HSM_DTHE to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_hsm_dthe(&self) -> MpuRdHsmDtheR {
        MpuRdHsmDtheR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask Interrupt from MPU_MSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_mss_mbox(&self) -> MpuRdMssMboxR {
        MpuRdMssMboxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Mask Interrupt from MPU_MSS_PCRA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_mss_pcra(&self) -> MpuRdMssPcraR {
        MpuRdMssPcraR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Mask Interrupt from MPU_MSS_QSPI to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_mss_qspi(&self) -> MpuRdMssQspiR {
        MpuRdMssQspiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Mask Interrupt from MPU_MSS_CR5A_AXIS to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_mss_cr5a_axis(&self) -> MpuRdMssCr5aAxisR {
        MpuRdMssCr5aAxisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Mask Interrupt from MPU_MSS_L2_BANKC to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_mss_cr5b_axis(&self) -> MpuRdMssCr5bAxisR {
        MpuRdMssCr5bAxisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Mask Interrupt from MPU_DSS_L3_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_dss_l3_banka(&self) -> MpuRdDssL3BankaR {
        MpuRdDssL3BankaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Mask Interrupt from MPU_DSS_L3_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_dss_l3_bankb(&self) -> MpuRdDssL3BankbR {
        MpuRdDssL3BankbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn mpu_rd_dss_l3_bankc(&self) -> MpuRdDssL3BankcR {
        MpuRdDssL3BankcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn mpu_rd_dss_l3_bankd(&self) -> MpuRdDssL3BankdR {
        MpuRdDssL3BankdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn mpu_rd_dss_hwa_dma0(&self) -> MpuRdDssHwaDma0R {
        MpuRdDssHwaDma0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
MRESERVED:Dont Use"]
    #[inline(always)]
    pub fn mpu_rd_dss_hwa_dma1(&self) -> MpuRdDssHwaDma1R {
        MpuRdDssHwaDma1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn mpu_rd_dss_hwa_proc(&self) -> MpuRdDssHwaProcR {
        MpuRdDssHwaProcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Mask Interrupt from MPU_DSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_dss_mbox(&self) -> MpuRdDssMboxR {
        MpuRdDssMboxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Interrupt from MPU_DSS_HSM to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn mpu_rd_hsm(&self) -> MpuRdHsmR {
        MpuRdHsmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Mask Interrupt from MPU_MSS_L2_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_mss_l2_banka(&mut self) -> MpuRdMssL2BankaW<MssPeriphErraggMask1Spec> {
        MpuRdMssL2BankaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Mask Interrupt from MPU_MSS_L2_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_mss_l2_bankb(&mut self) -> MpuRdMssL2BankbW<MssPeriphErraggMask1Spec> {
        MpuRdMssL2BankbW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Mask Interrupt from MPU_HSM_DTHE to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_hsm_dthe(&mut self) -> MpuRdHsmDtheW<MssPeriphErraggMask1Spec> {
        MpuRdHsmDtheW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask Interrupt from MPU_MSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_mss_mbox(&mut self) -> MpuRdMssMboxW<MssPeriphErraggMask1Spec> {
        MpuRdMssMboxW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Mask Interrupt from MPU_MSS_PCRA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_mss_pcra(&mut self) -> MpuRdMssPcraW<MssPeriphErraggMask1Spec> {
        MpuRdMssPcraW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Mask Interrupt from MPU_MSS_QSPI to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_mss_qspi(&mut self) -> MpuRdMssQspiW<MssPeriphErraggMask1Spec> {
        MpuRdMssQspiW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Mask Interrupt from MPU_MSS_CR5A_AXIS to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_mss_cr5a_axis(&mut self) -> MpuRdMssCr5aAxisW<MssPeriphErraggMask1Spec> {
        MpuRdMssCr5aAxisW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Mask Interrupt from MPU_MSS_L2_BANKC to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_mss_cr5b_axis(&mut self) -> MpuRdMssCr5bAxisW<MssPeriphErraggMask1Spec> {
        MpuRdMssCr5bAxisW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Mask Interrupt from MPU_DSS_L3_BANKA to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_l3_banka(&mut self) -> MpuRdDssL3BankaW<MssPeriphErraggMask1Spec> {
        MpuRdDssL3BankaW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Mask Interrupt from MPU_DSS_L3_BANKB to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_l3_bankb(&mut self) -> MpuRdDssL3BankbW<MssPeriphErraggMask1Spec> {
        MpuRdDssL3BankbW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_l3_bankc(&mut self) -> MpuRdDssL3BankcW<MssPeriphErraggMask1Spec> {
        MpuRdDssL3BankcW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_l3_bankd(&mut self) -> MpuRdDssL3BankdW<MssPeriphErraggMask1Spec> {
        MpuRdDssL3BankdW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_hwa_dma0(&mut self) -> MpuRdDssHwaDma0W<MssPeriphErraggMask1Spec> {
        MpuRdDssHwaDma0W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
MRESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_hwa_dma1(&mut self) -> MpuRdDssHwaDma1W<MssPeriphErraggMask1Spec> {
        MpuRdDssHwaDma1W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_hwa_proc(&mut self) -> MpuRdDssHwaProcW<MssPeriphErraggMask1Spec> {
        MpuRdDssHwaProcW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Mask Interrupt from MPU_DSS_MBOX to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_dss_mbox(&mut self) -> MpuRdDssMboxW<MssPeriphErraggMask1Spec> {
        MpuRdDssMboxW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Interrupt from MPU_DSS_HSM to aggregated Interrupt MSS_PERIPH_ACCESS_ERRAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rd_hsm(&mut self) -> MpuRdHsmW<MssPeriphErraggMask1Spec> {
        MpuRdHsmW::new(self, 16)
    }
}
#[doc = "MSS_PERIPH_ERRAGG_MASK1\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_mask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_mask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssPeriphErraggMask1Spec;
impl crate::RegisterSpec for MssPeriphErraggMask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_periph_erragg_mask1::R`](R) reader structure"]
impl crate::Readable for MssPeriphErraggMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_periph_erragg_mask1::W`](W) writer structure"]
impl crate::Writable for MssPeriphErraggMask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_PERIPH_ERRAGG_MASK1 to value 0"]
impl crate::Resettable for MssPeriphErraggMask1Spec {
    const RESET_VALUE: u32 = 0;
}
