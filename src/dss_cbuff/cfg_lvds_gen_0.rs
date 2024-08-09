#[doc = "Register `CFG_LVDS_GEN_0` reader"]
pub type R = crate::R<CfgLvdsGen0Spec>;
#[doc = "Register `CFG_LVDS_GEN_0` writer"]
pub type W = crate::W<CfgLvdsGen0Spec>;
#[doc = "Field `CFG_LVDS_LANE0_EN` reader - 0:0\\]
LVDS only programming : 0 : LVDS Lane 0 is disbaled 1 : LVDS Lane 0 is enabled"]
pub type CfgLvdsLane0EnR = crate::BitReader;
#[doc = "Field `CFG_LVDS_LANE0_EN` writer - 0:0\\]
LVDS only programming : 0 : LVDS Lane 0 is disbaled 1 : LVDS Lane 0 is enabled"]
pub type CfgLvdsLane0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_LVDS_LANE1_EN` reader - 1:1\\]
LVDS only programming : 0 : LVDS Lane 1 is disbaled 1 : LVDS Lane 1 is enabled"]
pub type CfgLvdsLane1EnR = crate::BitReader;
#[doc = "Field `CFG_LVDS_LANE1_EN` writer - 1:1\\]
LVDS only programming : 0 : LVDS Lane 1 is disbaled 1 : LVDS Lane 1 is enabled"]
pub type CfgLvdsLane1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_LVDS_LANE2_EN` reader - 2:2\\]
LVDS only programming : 0 : LVDS Lane 2 is disbaled 1 : LVDS Lane 2 is enabled"]
pub type CfgLvdsLane2EnR = crate::BitReader;
#[doc = "Field `CFG_LVDS_LANE2_EN` writer - 2:2\\]
LVDS only programming : 0 : LVDS Lane 2 is disbaled 1 : LVDS Lane 2 is enabled"]
pub type CfgLvdsLane2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_LVDS_LANE3_EN` reader - 3:3\\]
LVDS only programming : 0 : LVDS Lane 3 is disbaled 1 : LVDS Lane 3 is enabled"]
pub type CfgLvdsLane3EnR = crate::BitReader;
#[doc = "Field `CFG_LVDS_LANE3_EN` writer - 3:3\\]
LVDS only programming : 0 : LVDS Lane 3 is disbaled 1 : LVDS Lane 3 is enabled"]
pub type CfgLvdsLane3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_8B10B_EN` reader - 4:4\\]
TI Internal Feature. Reserved. For Furture enhancement. Not supported in this version 0 : No encoding 1: 8B10B encoding"]
pub type Cfg8b10bEnR = crate::BitReader;
#[doc = "Field `CFG_8B10B_EN` writer - 4:4\\]
TI Internal Feature. Reserved. For Furture enhancement. Not supported in this version 0 : No encoding 1: 8B10B encoding"]
pub type Cfg8b10bEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctc2en` reader - 5:5\\]
TI Internal feature. 0 : Regular operation 1: TC2MODE Enable (Not supported internally also in AR16xx)"]
pub type Ctc2enR = crate::BitReader;
#[doc = "Field `ctc2en` writer - 5:5\\]
TI Internal feature. 0 : Regular operation 1: TC2MODE Enable (Not supported internally also in AR16xx)"]
pub type Ctc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cacdsel` reader - 6:6\\]
TI Internal feature. CFG_ALL_CHL_READY_DELAY_SEL This bit is added to take of the fast to slow transition in the ADC Buffer. 0 => If the LVDS clock frequency (SDR) is >= 200MHz 1 => If the LVDS clock frequency (SDR) is &lt; 200MHz"]
pub type CacdselR = crate::BitReader;
#[doc = "Field `cacdsel` writer - 6:6\\]
TI Internal feature. CFG_ALL_CHL_READY_DELAY_SEL This bit is added to take of the fast to slow transition in the ADC Buffer. 0 => If the LVDS clock frequency (SDR) is >= 200MHz 1 => If the LVDS clock frequency (SDR) is &lt; 200MHz"]
pub type CacdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpkfmt` reader - 7:7\\]
TI Internal feature. CFG_PACK_FORMAT: While packing in 12/14 bit whether to use CSI like packing or general packing."]
pub type CpkfmtR = crate::BitReader;
#[doc = "Field `cpkfmt` writer - 7:7\\]
TI Internal feature. CFG_PACK_FORMAT: While packing in 12/14 bit whether to use CSI like packing or general packing."]
pub type CpkfmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_LINE_MODE` reader - 9:8\\]
TI Internal feature. Reserved."]
pub type CfgLineModeR = crate::FieldReader;
#[doc = "Field `CFG_LINE_MODE` writer - 9:8\\]
TI Internal feature. Reserved."]
pub type CfgLineModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CFG_BIT_CLK_MODE` reader - 10:10\\]
Bit Clock Mode 0 : SDR clocking mode 1 : DDR clocking mode"]
pub type CfgBitClkModeR = crate::BitReader;
#[doc = "Field `CFG_BIT_CLK_MODE` writer - 10:10\\]
Bit Clock Mode 0 : SDR clocking mode 1 : DDR clocking mode"]
pub type CfgBitClkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ccsmen` reader - 11:11\\]
TRM Description : As per alignment TI Restricted Description : 0 : Regular operation 1 : Continuous Streaming Mode Enabled (Not supported internally also in AR16xx)"]
pub type CcsmenR = crate::BitReader;
#[doc = "Field `ccsmen` writer - 11:11\\]
TRM Description : As per alignment TI Restricted Description : 0 : Regular operation 1 : Continuous Streaming Mode Enabled (Not supported internally also in AR16xx)"]
pub type CcsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ckchar` reader - 13:12\\]
TI Internal feature. CFG_K_CHAR_SEL"]
pub type CkcharR = crate::FieldReader;
#[doc = "Field `ckchar` writer - 13:12\\]
TI Internal feature. CFG_K_CHAR_SEL"]
pub type CkcharW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cclksel` reader - 14:14\\]
TI Internal feature. CFG_LVDS_CLK_SEL (between div-by-N and CLK_HSI_DIG) 1 -> CLK_HSI_DIG; 0 - through div-by-N (N is programmed in CFG_LVDS_CLK_DIV)"]
pub type CclkselR = crate::BitReader;
#[doc = "Field `cclksel` writer - 14:14\\]
TI Internal feature. CFG_LVDS_CLK_SEL (between div-by-N and CLK_HSI_DIG) 1 -> CLK_HSI_DIG; 0 - through div-by-N (N is programmed in CFG_LVDS_CLK_DIV)"]
pub type CclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cclksel1` reader - 15:15\\]
TRM Description : 0 : DDR mode clock mux 1 : SDR mode clock mux TI Restricted Description : CFG_LVDS_CLK_SEL1 0-> Use div-by-2 (Q2 path ) 1 -> Used for direct (Q1 path)"]
pub type Cclksel1R = crate::BitReader;
#[doc = "Field `cclksel1` writer - 15:15\\]
TRM Description : 0 : DDR mode clock mux 1 : SDR mode clock mux TI Restricted Description : CFG_LVDS_CLK_SEL1 0-> Use div-by-2 (Q2 path ) 1 -> Used for direct (Q1 path)"]
pub type Cclksel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cckdiv` reader - 21:16\\]
TI Internal feature. CFG_LVDS_CLK_DIV"]
pub type CckdivR = crate::FieldReader;
#[doc = "Field `cckdiv` writer - 21:16\\]
TI Internal feature. CFG_LVDS_CLK_DIV"]
pub type CckdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `cpossel` reader - 22:22\\]
0 : When a new chirp is starting, align first sample start to negedge of DDR clock. 1 : When a new chirp is starting, align first sample start to posedge of DDR clock (recommended)"]
pub type CposselR = crate::BitReader;
#[doc = "Field `cpossel` writer - 22:22\\]
0 : When a new chirp is starting, align first sample start to negedge of DDR clock. 1 : When a new chirp is starting, align first sample start to posedge of DDR clock (recommended)"]
pub type CposselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmsbf` reader - 23:23\\]
1 : Data is sent out on the LVDS lane MSB first 0 : Data is sent out on the LVDS lane LSB first"]
pub type CmsbfR = crate::BitReader;
#[doc = "Field `cmsbf` writer - 23:23\\]
1 : Data is sent out on the LVDS lane MSB first 0 : Data is sent out on the LVDS lane LSB first"]
pub type CmsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfdly` reader - 27:24\\]
LVDS FIFO Initial Threshold. This is a Static configuration and sould be set to a fixed value as mention in the Programming model"]
pub type CfdlyR = crate::FieldReader;
#[doc = "Field `cfdly` writer - 27:24\\]
LVDS FIFO Initial Threshold. This is a Static configuration and sould be set to a fixed value as mention in the Programming model"]
pub type CfdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cbcrcen` reader - 28:28\\]
LVDS Frame CRC 0 : CRC is not sent at the end of LVDS Frame 1 : CRC is sent at the end of the LVDS Frame"]
pub type CbcrcenR = crate::BitReader;
#[doc = "Field `cbcrcen` writer - 28:28\\]
LVDS Frame CRC 0 : CRC is not sent at the end of LVDS Frame 1 : CRC is sent at the end of the LVDS Frame"]
pub type CbcrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cblpen` reader - 29:29\\]
TI Internal CFG_LASTPULSE_EN"]
pub type CblpenR = crate::BitReader;
#[doc = "Field `cblpen` writer - 29:29\\]
TI Internal CFG_LASTPULSE_EN"]
pub type CblpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpz` reader - 31:30\\]
LVDS Clock config. 1 : Clock alignment enabled Others : Internal clock alignment not enabled This needs to be set to 0x1 for correct functionality"]
pub type CpzR = crate::FieldReader;
#[doc = "Field `cpz` writer - 31:30\\]
LVDS Clock config. 1 : Clock alignment enabled Others : Internal clock alignment not enabled This needs to be set to 0x1 for correct functionality"]
pub type CpzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
LVDS only programming : 0 : LVDS Lane 0 is disbaled 1 : LVDS Lane 0 is enabled"]
    #[inline(always)]
    pub fn cfg_lvds_lane0_en(&self) -> CfgLvdsLane0EnR {
        CfgLvdsLane0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
LVDS only programming : 0 : LVDS Lane 1 is disbaled 1 : LVDS Lane 1 is enabled"]
    #[inline(always)]
    pub fn cfg_lvds_lane1_en(&self) -> CfgLvdsLane1EnR {
        CfgLvdsLane1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
LVDS only programming : 0 : LVDS Lane 2 is disbaled 1 : LVDS Lane 2 is enabled"]
    #[inline(always)]
    pub fn cfg_lvds_lane2_en(&self) -> CfgLvdsLane2EnR {
        CfgLvdsLane2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
LVDS only programming : 0 : LVDS Lane 3 is disbaled 1 : LVDS Lane 3 is enabled"]
    #[inline(always)]
    pub fn cfg_lvds_lane3_en(&self) -> CfgLvdsLane3EnR {
        CfgLvdsLane3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feature. Reserved. For Furture enhancement. Not supported in this version 0 : No encoding 1: 8B10B encoding"]
    #[inline(always)]
    pub fn cfg_8b10b_en(&self) -> Cfg8b10bEnR {
        Cfg8b10bEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal feature. 0 : Regular operation 1: TC2MODE Enable (Not supported internally also in AR16xx)"]
    #[inline(always)]
    pub fn ctc2en(&self) -> Ctc2enR {
        Ctc2enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
TI Internal feature. CFG_ALL_CHL_READY_DELAY_SEL This bit is added to take of the fast to slow transition in the ADC Buffer. 0 => If the LVDS clock frequency (SDR) is >= 200MHz 1 => If the LVDS clock frequency (SDR) is &lt; 200MHz"]
    #[inline(always)]
    pub fn cacdsel(&self) -> CacdselR {
        CacdselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
TI Internal feature. CFG_PACK_FORMAT: While packing in 12/14 bit whether to use CSI like packing or general packing."]
    #[inline(always)]
    pub fn cpkfmt(&self) -> CpkfmtR {
        CpkfmtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
TI Internal feature. Reserved."]
    #[inline(always)]
    pub fn cfg_line_mode(&self) -> CfgLineModeR {
        CfgLineModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
Bit Clock Mode 0 : SDR clocking mode 1 : DDR clocking mode"]
    #[inline(always)]
    pub fn cfg_bit_clk_mode(&self) -> CfgBitClkModeR {
        CfgBitClkModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
TRM Description : As per alignment TI Restricted Description : 0 : Regular operation 1 : Continuous Streaming Mode Enabled (Not supported internally also in AR16xx)"]
    #[inline(always)]
    pub fn ccsmen(&self) -> CcsmenR {
        CcsmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
TI Internal feature. CFG_K_CHAR_SEL"]
    #[inline(always)]
    pub fn ckchar(&self) -> CkcharR {
        CkcharR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
TI Internal feature. CFG_LVDS_CLK_SEL (between div-by-N and CLK_HSI_DIG) 1 -> CLK_HSI_DIG; 0 - through div-by-N (N is programmed in CFG_LVDS_CLK_DIV)"]
    #[inline(always)]
    pub fn cclksel(&self) -> CclkselR {
        CclkselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
TRM Description : 0 : DDR mode clock mux 1 : SDR mode clock mux TI Restricted Description : CFG_LVDS_CLK_SEL1 0-> Use div-by-2 (Q2 path ) 1 -> Used for direct (Q1 path)"]
    #[inline(always)]
    pub fn cclksel1(&self) -> Cclksel1R {
        Cclksel1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
TI Internal feature. CFG_LVDS_CLK_DIV"]
    #[inline(always)]
    pub fn cckdiv(&self) -> CckdivR {
        CckdivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
0 : When a new chirp is starting, align first sample start to negedge of DDR clock. 1 : When a new chirp is starting, align first sample start to posedge of DDR clock (recommended)"]
    #[inline(always)]
    pub fn cpossel(&self) -> CposselR {
        CposselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
1 : Data is sent out on the LVDS lane MSB first 0 : Data is sent out on the LVDS lane LSB first"]
    #[inline(always)]
    pub fn cmsbf(&self) -> CmsbfR {
        CmsbfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
LVDS FIFO Initial Threshold. This is a Static configuration and sould be set to a fixed value as mention in the Programming model"]
    #[inline(always)]
    pub fn cfdly(&self) -> CfdlyR {
        CfdlyR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
LVDS Frame CRC 0 : CRC is not sent at the end of LVDS Frame 1 : CRC is sent at the end of the LVDS Frame"]
    #[inline(always)]
    pub fn cbcrcen(&self) -> CbcrcenR {
        CbcrcenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal CFG_LASTPULSE_EN"]
    #[inline(always)]
    pub fn cblpen(&self) -> CblpenR {
        CblpenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
LVDS Clock config. 1 : Clock alignment enabled Others : Internal clock alignment not enabled This needs to be set to 0x1 for correct functionality"]
    #[inline(always)]
    pub fn cpz(&self) -> CpzR {
        CpzR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LVDS only programming : 0 : LVDS Lane 0 is disbaled 1 : LVDS Lane 0 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_lane0_en(&mut self) -> CfgLvdsLane0EnW<CfgLvdsGen0Spec> {
        CfgLvdsLane0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
LVDS only programming : 0 : LVDS Lane 1 is disbaled 1 : LVDS Lane 1 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_lane1_en(&mut self) -> CfgLvdsLane1EnW<CfgLvdsGen0Spec> {
        CfgLvdsLane1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
LVDS only programming : 0 : LVDS Lane 2 is disbaled 1 : LVDS Lane 2 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_lane2_en(&mut self) -> CfgLvdsLane2EnW<CfgLvdsGen0Spec> {
        CfgLvdsLane2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
LVDS only programming : 0 : LVDS Lane 3 is disbaled 1 : LVDS Lane 3 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_lane3_en(&mut self) -> CfgLvdsLane3EnW<CfgLvdsGen0Spec> {
        CfgLvdsLane3EnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal Feature. Reserved. For Furture enhancement. Not supported in this version 0 : No encoding 1: 8B10B encoding"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_8b10b_en(&mut self) -> Cfg8b10bEnW<CfgLvdsGen0Spec> {
        Cfg8b10bEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal feature. 0 : Regular operation 1: TC2MODE Enable (Not supported internally also in AR16xx)"]
    #[inline(always)]
    #[must_use]
    pub fn ctc2en(&mut self) -> Ctc2enW<CfgLvdsGen0Spec> {
        Ctc2enW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
TI Internal feature. CFG_ALL_CHL_READY_DELAY_SEL This bit is added to take of the fast to slow transition in the ADC Buffer. 0 => If the LVDS clock frequency (SDR) is >= 200MHz 1 => If the LVDS clock frequency (SDR) is &lt; 200MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cacdsel(&mut self) -> CacdselW<CfgLvdsGen0Spec> {
        CacdselW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
TI Internal feature. CFG_PACK_FORMAT: While packing in 12/14 bit whether to use CSI like packing or general packing."]
    #[inline(always)]
    #[must_use]
    pub fn cpkfmt(&mut self) -> CpkfmtW<CfgLvdsGen0Spec> {
        CpkfmtW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
TI Internal feature. Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_line_mode(&mut self) -> CfgLineModeW<CfgLvdsGen0Spec> {
        CfgLineModeW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
Bit Clock Mode 0 : SDR clocking mode 1 : DDR clocking mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_bit_clk_mode(&mut self) -> CfgBitClkModeW<CfgLvdsGen0Spec> {
        CfgBitClkModeW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
TRM Description : As per alignment TI Restricted Description : 0 : Regular operation 1 : Continuous Streaming Mode Enabled (Not supported internally also in AR16xx)"]
    #[inline(always)]
    #[must_use]
    pub fn ccsmen(&mut self) -> CcsmenW<CfgLvdsGen0Spec> {
        CcsmenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
TI Internal feature. CFG_K_CHAR_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ckchar(&mut self) -> CkcharW<CfgLvdsGen0Spec> {
        CkcharW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
TI Internal feature. CFG_LVDS_CLK_SEL (between div-by-N and CLK_HSI_DIG) 1 -> CLK_HSI_DIG; 0 - through div-by-N (N is programmed in CFG_LVDS_CLK_DIV)"]
    #[inline(always)]
    #[must_use]
    pub fn cclksel(&mut self) -> CclkselW<CfgLvdsGen0Spec> {
        CclkselW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
TRM Description : 0 : DDR mode clock mux 1 : SDR mode clock mux TI Restricted Description : CFG_LVDS_CLK_SEL1 0-> Use div-by-2 (Q2 path ) 1 -> Used for direct (Q1 path)"]
    #[inline(always)]
    #[must_use]
    pub fn cclksel1(&mut self) -> Cclksel1W<CfgLvdsGen0Spec> {
        Cclksel1W::new(self, 15)
    }
    #[doc = "Bits 16:21 - 21:16\\]
TI Internal feature. CFG_LVDS_CLK_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn cckdiv(&mut self) -> CckdivW<CfgLvdsGen0Spec> {
        CckdivW::new(self, 16)
    }
    #[doc = "Bit 22 - 22:22\\]
0 : When a new chirp is starting, align first sample start to negedge of DDR clock. 1 : When a new chirp is starting, align first sample start to posedge of DDR clock (recommended)"]
    #[inline(always)]
    #[must_use]
    pub fn cpossel(&mut self) -> CposselW<CfgLvdsGen0Spec> {
        CposselW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
1 : Data is sent out on the LVDS lane MSB first 0 : Data is sent out on the LVDS lane LSB first"]
    #[inline(always)]
    #[must_use]
    pub fn cmsbf(&mut self) -> CmsbfW<CfgLvdsGen0Spec> {
        CmsbfW::new(self, 23)
    }
    #[doc = "Bits 24:27 - 27:24\\]
LVDS FIFO Initial Threshold. This is a Static configuration and sould be set to a fixed value as mention in the Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn cfdly(&mut self) -> CfdlyW<CfgLvdsGen0Spec> {
        CfdlyW::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
LVDS Frame CRC 0 : CRC is not sent at the end of LVDS Frame 1 : CRC is sent at the end of the LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn cbcrcen(&mut self) -> CbcrcenW<CfgLvdsGen0Spec> {
        CbcrcenW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal CFG_LASTPULSE_EN"]
    #[inline(always)]
    #[must_use]
    pub fn cblpen(&mut self) -> CblpenW<CfgLvdsGen0Spec> {
        CblpenW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
LVDS Clock config. 1 : Clock alignment enabled Others : Internal clock alignment not enabled This needs to be set to 0x1 for correct functionality"]
    #[inline(always)]
    #[must_use]
    pub fn cpz(&mut self) -> CpzW<CfgLvdsGen0Spec> {
        CpzW::new(self, 30)
    }
}
#[doc = "CFG_LVDS_GEN_0\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_gen_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_gen_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLvdsGen0Spec;
impl crate::RegisterSpec for CfgLvdsGen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lvds_gen_0::R`](R) reader structure"]
impl crate::Readable for CfgLvdsGen0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_lvds_gen_0::W`](W) writer structure"]
impl crate::Writable for CfgLvdsGen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LVDS_GEN_0 to value 0"]
impl crate::Resettable for CfgLvdsGen0Spec {
    const RESET_VALUE: u32 = 0;
}
