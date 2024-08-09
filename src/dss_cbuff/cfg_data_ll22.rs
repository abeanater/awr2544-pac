#[doc = "Register `CFG_DATA_LL22` reader"]
pub type R = crate::R<CfgDataLl22Spec>;
#[doc = "Register `CFG_DATA_LL22` writer"]
pub type W = crate::W<CfgDataLl22Spec>;
#[doc = "Field `LL22_VALID` reader - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
pub type Ll22ValidR = crate::BitReader;
#[doc = "Field `LL22_VALID` writer - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
pub type Ll22ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_HE` reader - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
pub type Ll22HeR = crate::BitReader;
#[doc = "Field `LL22_HE` writer - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
pub type Ll22HeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_HS` reader - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
pub type Ll22HsR = crate::BitReader;
#[doc = "Field `LL22_HS` writer - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
pub type Ll22HsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_VCNUM` reader - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
pub type Ll22VcnumR = crate::FieldReader;
#[doc = "Field `LL22_VCNUM` writer - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
pub type Ll22VcnumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LL22_FMT` reader - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
pub type Ll22FmtR = crate::FieldReader;
#[doc = "Field `LL22_FMT` writer - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
pub type Ll22FmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LL22_FMT_MAP` reader - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
pub type Ll22FmtMapR = crate::BitReader;
#[doc = "Field `LL22_FMT_MAP` writer - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
pub type Ll22FmtMapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_FMT_IN` reader - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
pub type Ll22FmtInR = crate::BitReader;
#[doc = "Field `LL22_FMT_IN` writer - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
pub type Ll22FmtInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_SIZE` reader - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
pub type Ll22SizeR = crate::FieldReader<u16>;
#[doc = "Field `LL22_SIZE` writer - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
pub type Ll22SizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LL22_BITPOS_SEL` reader - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
pub type Ll22BitposSelR = crate::FieldReader;
#[doc = "Field `LL22_BITPOS_SEL` writer - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
pub type Ll22BitposSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LL22_WAITFOR_PKTSENT` reader - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
pub type Ll22WaitforPktsentR = crate::BitReader;
#[doc = "Field `LL22_WAITFOR_PKTSENT` writer - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
pub type Ll22WaitforPktsentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_LPHDR_EN` reader - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
pub type Ll22LphdrEnR = crate::BitReader;
#[doc = "Field `LL22_LPHDR_EN` writer - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
pub type Ll22LphdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_CRC_EN` reader - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
pub type Ll22CrcEnR = crate::BitReader;
#[doc = "Field `LL22_CRC_EN` writer - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
pub type Ll22CrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_SHORT_PKT_DELAY_EN` reader - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll22ShortPktDelayEnR = crate::BitReader;
#[doc = "Field `LL22_SHORT_PKT_DELAY_EN` writer - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll22ShortPktDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_LONG_PKT_DELAY_EN` reader - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll22LongPktDelayEnR = crate::BitReader;
#[doc = "Field `LL22_LONG_PKT_DELAY_EN` writer - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll22LongPktDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL22_DATA_WR_DELAY_EN` reader - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll22DataWrDelayEnR = crate::BitReader;
#[doc = "Field `LL22_DATA_WR_DELAY_EN` writer - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll22DataWrDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
    #[inline(always)]
    pub fn ll22_valid(&self) -> Ll22ValidR {
        Ll22ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
    #[inline(always)]
    pub fn ll22_he(&self) -> Ll22HeR {
        Ll22HeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
    #[inline(always)]
    pub fn ll22_hs(&self) -> Ll22HsR {
        Ll22HsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
    #[inline(always)]
    pub fn ll22_vcnum(&self) -> Ll22VcnumR {
        Ll22VcnumR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
    #[inline(always)]
    pub fn ll22_fmt(&self) -> Ll22FmtR {
        Ll22FmtR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
    #[inline(always)]
    pub fn ll22_fmt_map(&self) -> Ll22FmtMapR {
        Ll22FmtMapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
    #[inline(always)]
    pub fn ll22_fmt_in(&self) -> Ll22FmtInR {
        Ll22FmtInR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:22 - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
    #[inline(always)]
    pub fn ll22_size(&self) -> Ll22SizeR {
        Ll22SizeR::new(((self.bits >> 9) & 0x3fff) as u16)
    }
    #[doc = "Bits 23:25 - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
    #[inline(always)]
    pub fn ll22_bitpos_sel(&self) -> Ll22BitposSelR {
        Ll22BitposSelR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
    #[inline(always)]
    pub fn ll22_waitfor_pktsent(&self) -> Ll22WaitforPktsentR {
        Ll22WaitforPktsentR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
    #[inline(always)]
    pub fn ll22_lphdr_en(&self) -> Ll22LphdrEnR {
        Ll22LphdrEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
    #[inline(always)]
    pub fn ll22_crc_en(&self) -> Ll22CrcEnR {
        Ll22CrcEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll22_short_pkt_delay_en(&self) -> Ll22ShortPktDelayEnR {
        Ll22ShortPktDelayEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll22_long_pkt_delay_en(&self) -> Ll22LongPktDelayEnR {
        Ll22LongPktDelayEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll22_data_wr_delay_en(&self) -> Ll22DataWrDelayEnR {
        Ll22DataWrDelayEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_valid(&mut self) -> Ll22ValidW<CfgDataLl22Spec> {
        Ll22ValidW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_he(&mut self) -> Ll22HeW<CfgDataLl22Spec> {
        Ll22HeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_hs(&mut self) -> Ll22HsW<CfgDataLl22Spec> {
        Ll22HsW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_vcnum(&mut self) -> Ll22VcnumW<CfgDataLl22Spec> {
        Ll22VcnumW::new(self, 3)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_fmt(&mut self) -> Ll22FmtW<CfgDataLl22Spec> {
        Ll22FmtW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_fmt_map(&mut self) -> Ll22FmtMapW<CfgDataLl22Spec> {
        Ll22FmtMapW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_fmt_in(&mut self) -> Ll22FmtInW<CfgDataLl22Spec> {
        Ll22FmtInW::new(self, 8)
    }
    #[doc = "Bits 9:22 - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_size(&mut self) -> Ll22SizeW<CfgDataLl22Spec> {
        Ll22SizeW::new(self, 9)
    }
    #[doc = "Bits 23:25 - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_bitpos_sel(&mut self) -> Ll22BitposSelW<CfgDataLl22Spec> {
        Ll22BitposSelW::new(self, 23)
    }
    #[doc = "Bit 26 - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_waitfor_pktsent(&mut self) -> Ll22WaitforPktsentW<CfgDataLl22Spec> {
        Ll22WaitforPktsentW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_lphdr_en(&mut self) -> Ll22LphdrEnW<CfgDataLl22Spec> {
        Ll22LphdrEnW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_crc_en(&mut self) -> Ll22CrcEnW<CfgDataLl22Spec> {
        Ll22CrcEnW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_short_pkt_delay_en(&mut self) -> Ll22ShortPktDelayEnW<CfgDataLl22Spec> {
        Ll22ShortPktDelayEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_long_pkt_delay_en(&mut self) -> Ll22LongPktDelayEnW<CfgDataLl22Spec> {
        Ll22LongPktDelayEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll22_data_wr_delay_en(&mut self) -> Ll22DataWrDelayEnW<CfgDataLl22Spec> {
        Ll22DataWrDelayEnW::new(self, 31)
    }
}
#[doc = "CFG_DATA_LL22\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDataLl22Spec;
impl crate::RegisterSpec for CfgDataLl22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data_ll22::R`](R) reader structure"]
impl crate::Readable for CfgDataLl22Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_data_ll22::W`](W) writer structure"]
impl crate::Writable for CfgDataLl22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DATA_LL22 to value 0"]
impl crate::Resettable for CfgDataLl22Spec {
    const RESET_VALUE: u32 = 0;
}
