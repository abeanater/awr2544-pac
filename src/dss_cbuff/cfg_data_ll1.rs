#[doc = "Register `CFG_DATA_LL1` reader"]
pub type R = crate::R<CfgDataLl1Spec>;
#[doc = "Register `CFG_DATA_LL1` writer"]
pub type W = crate::W<CfgDataLl1Spec>;
#[doc = "Field `LL1_VALID` reader - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
pub type Ll1ValidR = crate::BitReader;
#[doc = "Field `LL1_VALID` writer - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
pub type Ll1ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_HE` reader - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
pub type Ll1HeR = crate::BitReader;
#[doc = "Field `LL1_HE` writer - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
pub type Ll1HeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_HS` reader - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
pub type Ll1HsR = crate::BitReader;
#[doc = "Field `LL1_HS` writer - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
pub type Ll1HsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_VCNUM` reader - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
pub type Ll1VcnumR = crate::FieldReader;
#[doc = "Field `LL1_VCNUM` writer - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
pub type Ll1VcnumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LL1_FMT` reader - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
pub type Ll1FmtR = crate::FieldReader;
#[doc = "Field `LL1_FMT` writer - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
pub type Ll1FmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LL1_FMT_MAP` reader - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
pub type Ll1FmtMapR = crate::BitReader;
#[doc = "Field `LL1_FMT_MAP` writer - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
pub type Ll1FmtMapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_FMT_IN` reader - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
pub type Ll1FmtInR = crate::BitReader;
#[doc = "Field `LL1_FMT_IN` writer - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
pub type Ll1FmtInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_SIZE` reader - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
pub type Ll1SizeR = crate::FieldReader<u16>;
#[doc = "Field `LL1_SIZE` writer - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
pub type Ll1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LL1_BITPOS_SEL` reader - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
pub type Ll1BitposSelR = crate::FieldReader;
#[doc = "Field `LL1_BITPOS_SEL` writer - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
pub type Ll1BitposSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LL1_WAITFOR_PKTSENT` reader - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
pub type Ll1WaitforPktsentR = crate::BitReader;
#[doc = "Field `LL1_WAITFOR_PKTSENT` writer - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
pub type Ll1WaitforPktsentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_LPHDR_EN` reader - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
pub type Ll1LphdrEnR = crate::BitReader;
#[doc = "Field `LL1_LPHDR_EN` writer - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
pub type Ll1LphdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_CRC_EN` reader - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
pub type Ll1CrcEnR = crate::BitReader;
#[doc = "Field `LL1_CRC_EN` writer - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
pub type Ll1CrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_SHORT_PKT_DELAY_EN` reader - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll1ShortPktDelayEnR = crate::BitReader;
#[doc = "Field `LL1_SHORT_PKT_DELAY_EN` writer - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll1ShortPktDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_LONG_PKT_DELAY_EN` reader - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll1LongPktDelayEnR = crate::BitReader;
#[doc = "Field `LL1_LONG_PKT_DELAY_EN` writer - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll1LongPktDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL1_DATA_WR_DELAY_EN` reader - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll1DataWrDelayEnR = crate::BitReader;
#[doc = "Field `LL1_DATA_WR_DELAY_EN` writer - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
pub type Ll1DataWrDelayEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
    #[inline(always)]
    pub fn ll1_valid(&self) -> Ll1ValidR {
        Ll1ValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
    #[inline(always)]
    pub fn ll1_he(&self) -> Ll1HeR {
        Ll1HeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
    #[inline(always)]
    pub fn ll1_hs(&self) -> Ll1HsR {
        Ll1HsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
    #[inline(always)]
    pub fn ll1_vcnum(&self) -> Ll1VcnumR {
        Ll1VcnumR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
    #[inline(always)]
    pub fn ll1_fmt(&self) -> Ll1FmtR {
        Ll1FmtR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
    #[inline(always)]
    pub fn ll1_fmt_map(&self) -> Ll1FmtMapR {
        Ll1FmtMapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
    #[inline(always)]
    pub fn ll1_fmt_in(&self) -> Ll1FmtInR {
        Ll1FmtInR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:22 - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
    #[inline(always)]
    pub fn ll1_size(&self) -> Ll1SizeR {
        Ll1SizeR::new(((self.bits >> 9) & 0x3fff) as u16)
    }
    #[doc = "Bits 23:25 - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
    #[inline(always)]
    pub fn ll1_bitpos_sel(&self) -> Ll1BitposSelR {
        Ll1BitposSelR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
    #[inline(always)]
    pub fn ll1_waitfor_pktsent(&self) -> Ll1WaitforPktsentR {
        Ll1WaitforPktsentR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
    #[inline(always)]
    pub fn ll1_lphdr_en(&self) -> Ll1LphdrEnR {
        Ll1LphdrEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
    #[inline(always)]
    pub fn ll1_crc_en(&self) -> Ll1CrcEnR {
        Ll1CrcEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll1_short_pkt_delay_en(&self) -> Ll1ShortPktDelayEnR {
        Ll1ShortPktDelayEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll1_long_pkt_delay_en(&self) -> Ll1LongPktDelayEnR {
        Ll1LongPktDelayEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    pub fn ll1_data_wr_delay_en(&self) -> Ll1DataWrDelayEnR {
        Ll1DataWrDelayEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0 : Linklist entry is invalid 1 : Linklist entry is valid"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_valid(&mut self) -> Ll1ValidW<CfgDataLl1Spec> {
        Ll1ValidW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CSI-2 : 0 : Do not send an Hsync End packet after sending this data 1 : Send an Hsync End Packet after sending this data LVDS : 0 : Entry is not the last data of LVDS Frame 1 : Entry is the last data in the LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_he(&mut self) -> Ll1HeW<CfgDataLl1Spec> {
        Ll1HeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CSI-2 : 0 : Do not send an Hsync Start packet before sending this data 1 : Send an Hsync Start Packet before sending this data LVDS : 0 : Entry is not the first data of LVDS Frame 1 : Entry is the first data in the LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_hs(&mut self) -> Ll1HsW<CfgDataLl1Spec> {
        Ll1HsW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CSI-2 : Configure the Virtual Channel Number for the Long Packet over which this data is sent"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_vcnum(&mut self) -> Ll1VcnumW<CfgDataLl1Spec> {
        Ll1VcnumW::new(self, 3)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Specify the LVDS/CSI2 output format. 00 - 16bit 01 - 14-bit 10 - 12-bit"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_fmt(&mut self) -> Ll1FmtW<CfgDataLl1Spec> {
        Ll1FmtW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
LVDS only : 0 : Choose CFG_LVDS_MAPPING_LANEx_FMT_0_y 1 : Choose CFG_LVDS_MAPPING_LANEx_FMT_1_y"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_fmt_map(&mut self) -> Ll1FmtMapW<CfgDataLl1Spec> {
        Ll1FmtMapW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
0 : The incoming data sources for this Linklist is aligned to 128-bit 1 : The incoming data sources for this Linklist is aligned to 96-bit"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_fmt_in(&mut self) -> Ll1FmtInW<CfgDataLl1Spec> {
        Ll1FmtInW::new(self, 8)
    }
    #[doc = "Bits 9:22 - 22:9\\]
Configure the Size of the data in terms of the numer of samples (not in terms of number of bytes). Sample refers to a 16 bit CBUFF Unit"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_size(&mut self) -> Ll1SizeW<CfgDataLl1Spec> {
        Ll1SizeW::new(self, 9)
    }
    #[doc = "Bits 23:25 - 25:23\\]
TI Internal Feature. Reserved for future use to select which of the 12-bits or 14-bits to be picked up from 16-bit CBUFF unit"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_bitpos_sel(&mut self) -> Ll1BitposSelW<CfgDataLl1Spec> {
        Ll1BitposSelW::new(self, 23)
    }
    #[doc = "Bit 26 - 26:26\\]
TI Internal Feature Reserved for furture debug enhancement 1 : Wait for packet sent signal ack from CSI2 to move forward 0 : Do not wait for packet sent"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_waitfor_pktsent(&mut self) -> Ll1WaitforPktsentW<CfgDataLl1Spec> {
        Ll1WaitforPktsentW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
CSI2 Programming : 1 : Entry is start of a new CSI-2 packet. Send the LP Payload Header before sending data corresponind to this Linklist 0 : Link list is not the start of a Long packet but part of previous packet and hence directly send data LVDS Programming : 1 : Entry is start of a new LVDS Frame 0 : Entry is not the start of the new LVDS Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_lphdr_en(&mut self) -> Ll1LphdrEnW<CfgDataLl1Spec> {
        Ll1LphdrEnW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
0 : CRC is disbaled 1 : This linklist corresponds to ADC Buffer data. Enable the CRC check from ADC Buffer to CBUFF"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_crc_en(&mut self) -> Ll1CrcEnW<CfgDataLl1Spec> {
        Ll1CrcEnW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_short_pkt_delay_en(&mut self) -> Ll1ShortPktDelayEnW<CfgDataLl1Spec> {
        Ll1ShortPktDelayEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_long_pkt_delay_en(&mut self) -> Ll1LongPktDelayEnW<CfgDataLl1Spec> {
        Ll1LongPktDelayEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
TI Internal Feature CSI2 only Programming : Use the Packet Delay configured in CFG_DELAY_CONFIG. This is a Debug feature. Not requred in Programming model"]
    #[inline(always)]
    #[must_use]
    pub fn ll1_data_wr_delay_en(&mut self) -> Ll1DataWrDelayEnW<CfgDataLl1Spec> {
        Ll1DataWrDelayEnW::new(self, 31)
    }
}
#[doc = "CFG_DATA_LL1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDataLl1Spec;
impl crate::RegisterSpec for CfgDataLl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data_ll1::R`](R) reader structure"]
impl crate::Readable for CfgDataLl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_data_ll1::W`](W) writer structure"]
impl crate::Writable for CfgDataLl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DATA_LL1 to value 0"]
impl crate::Resettable for CfgDataLl1Spec {
    const RESET_VALUE: u32 = 0;
}
