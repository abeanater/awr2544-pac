#[doc = "Register `CFG_DATA_LL30` reader"]
pub type R = crate::R<CfgDataLl30Spec>;
#[doc = "Register `CFG_DATA_LL30` writer"]
pub type W = crate::W<CfgDataLl30Spec>;
#[doc = "Field `LL30_VALID` reader - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
pub type Ll30ValidR = crate::BitReader;
#[doc = "Field `LL30_VALID` writer - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
pub type Ll30ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_HE` reader - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
pub type Ll30HeR = crate::BitReader;
#[doc = "Field `LL30_HE` writer - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
pub type Ll30HeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_HS` reader - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
pub type Ll30HsR = crate::BitReader;
#[doc = "Field `LL30_HS` writer - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
pub type Ll30HsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_VCNUM` reader - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
pub type Ll30VcnumR = crate::FieldReader;
#[doc = "Field `LL30_VCNUM` writer - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
pub type Ll30VcnumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LL30_FMT` reader - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
pub type Ll30FmtR = crate::FieldReader;
#[doc = "Field `LL30_FMT` writer - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
pub type Ll30FmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LL30_FMT_MAP` reader - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
pub type Ll30FmtMapR = crate::BitReader;
#[doc = "Field `LL30_FMT_MAP` writer - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
pub type Ll30FmtMapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_FMT_IN` reader - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
pub type Ll30FmtInR = crate::BitReader;
#[doc = "Field `LL30_FMT_IN` writer - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
pub type Ll30FmtInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_SIZE` reader - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
pub type Ll30SizeR = crate::FieldReader<u16>;
#[doc = "Field `LL30_SIZE` writer - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
pub type Ll30SizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LL30_BITPOS_SEL` reader - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
pub type Ll30BitposSelR = crate::FieldReader;
#[doc = "Field `LL30_BITPOS_SEL` writer - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
pub type Ll30BitposSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LL30_WAITFOR_PKTSENT` reader - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
pub type Ll30WaitforPktsentR = crate::BitReader;
#[doc = "Field `LL30_WAITFOR_PKTSENT` writer - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
pub type Ll30WaitforPktsentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_LPHDR_EN` reader - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
pub type Ll30LphdrEnR = crate::BitReader;
#[doc = "Field `LL30_LPHDR_EN` writer - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
pub type Ll30LphdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_CRC_EN` reader - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
pub type Ll30CrcEnR = crate::BitReader;
#[doc = "Field `LL30_CRC_EN` writer - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
pub type Ll30CrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_SHORT_PKT_DELAY_EN` reader - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll30ShortPktDelayEnR = crate::BitReader;
#[doc = "Field `LL30_SHORT_PKT_DELAY_EN` writer - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll30ShortPktDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_LONG_PKT_DELAY_EN` reader - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll30LongPktDelayEnR = crate::BitReader;
#[doc = "Field `LL30_LONG_PKT_DELAY_EN` writer - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll30LongPktDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL30_DATA_WR_DELAY_EN` reader - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll30DataWrDelayEnR = crate::BitReader;
#[doc = "Field `LL30_DATA_WR_DELAY_EN` writer - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll30DataWrDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
    #[inline(always)]
    pub fn ll30_valid(&self) -> Ll30ValidR {
        Ll30ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
    #[inline(always)]
    pub fn ll30_he(&self) -> Ll30HeR {
        Ll30HeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
    #[inline(always)]
    pub fn ll30_hs(&self) -> Ll30HsR {
        Ll30HsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
    #[inline(always)]
    pub fn ll30_vcnum(&self) -> Ll30VcnumR {
        Ll30VcnumR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
    #[inline(always)]
    pub fn ll30_fmt(&self) -> Ll30FmtR {
        Ll30FmtR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
    #[inline(always)]
    pub fn ll30_fmt_map(&self) -> Ll30FmtMapR {
        Ll30FmtMapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
    #[inline(always)]
    pub fn ll30_fmt_in(&self) -> Ll30FmtInR {
        Ll30FmtInR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:22 - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
    #[inline(always)]
    pub fn ll30_size(&self) -> Ll30SizeR {
        Ll30SizeR::new(((self.bits >> 9) & 0x3fff) as u16)
    }
    #[doc = "Bits 23:25 - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
    #[inline(always)]
    pub fn ll30_bitpos_sel(&self) -> Ll30BitposSelR {
        Ll30BitposSelR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
    #[inline(always)]
    pub fn ll30_waitfor_pktsent(&self) -> Ll30WaitforPktsentR {
        Ll30WaitforPktsentR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
    #[inline(always)]
    pub fn ll30_lphdr_en(&self) -> Ll30LphdrEnR {
        Ll30LphdrEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
    #[inline(always)]
    pub fn ll30_crc_en(&self) -> Ll30CrcEnR {
        Ll30CrcEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll30_short_pkt_delay_en(&self) -> Ll30ShortPktDelayEnR {
        Ll30ShortPktDelayEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll30_long_pkt_delay_en(&self) -> Ll30LongPktDelayEnR {
        Ll30LongPktDelayEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll30_data_wr_delay_en(&self) -> Ll30DataWrDelayEnR {
        Ll30DataWrDelayEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_valid(&mut self) -> Ll30ValidW<CfgDataLl30Spec> {
        Ll30ValidW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_he(&mut self) -> Ll30HeW<CfgDataLl30Spec> {
        Ll30HeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_hs(&mut self) -> Ll30HsW<CfgDataLl30Spec> {
        Ll30HsW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_vcnum(&mut self) -> Ll30VcnumW<CfgDataLl30Spec> {
        Ll30VcnumW::new(self, 3)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_fmt(&mut self) -> Ll30FmtW<CfgDataLl30Spec> {
        Ll30FmtW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_fmt_map(&mut self) -> Ll30FmtMapW<CfgDataLl30Spec> {
        Ll30FmtMapW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_fmt_in(&mut self) -> Ll30FmtInW<CfgDataLl30Spec> {
        Ll30FmtInW::new(self, 8)
    }
    #[doc = "Bits 9:22 - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_size(&mut self) -> Ll30SizeW<CfgDataLl30Spec> {
        Ll30SizeW::new(self, 9)
    }
    #[doc = "Bits 23:25 - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_bitpos_sel(&mut self) -> Ll30BitposSelW<CfgDataLl30Spec> {
        Ll30BitposSelW::new(self, 23)
    }
    #[doc = "Bit 26 - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_waitfor_pktsent(&mut self) -> Ll30WaitforPktsentW<CfgDataLl30Spec> {
        Ll30WaitforPktsentW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_lphdr_en(&mut self) -> Ll30LphdrEnW<CfgDataLl30Spec> {
        Ll30LphdrEnW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_crc_en(&mut self) -> Ll30CrcEnW<CfgDataLl30Spec> {
        Ll30CrcEnW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_short_pkt_delay_en(&mut self) -> Ll30ShortPktDelayEnW<CfgDataLl30Spec> {
        Ll30ShortPktDelayEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_long_pkt_delay_en(&mut self) -> Ll30LongPktDelayEnW<CfgDataLl30Spec> {
        Ll30LongPktDelayEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_data_wr_delay_en(&mut self) -> Ll30DataWrDelayEnW<CfgDataLl30Spec> {
        Ll30DataWrDelayEnW::new(self, 31)
    }
}
#[doc = "CFG_DATA_LL30\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDataLl30Spec;
impl crate::RegisterSpec for CfgDataLl30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data_ll30::R`](R) reader structure"]
impl crate::Readable for CfgDataLl30Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_data_ll30::W`](W) writer structure"]
impl crate::Writable for CfgDataLl30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DATA_LL30 to value 0"]
impl crate::Resettable for CfgDataLl30Spec {
    const RESET_VALUE: u32 = 0;
}
