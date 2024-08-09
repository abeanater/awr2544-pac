#[doc = "Register `CFG_DATA_LL2_THRESHOLD` reader"]
pub type R = crate::R<CfgDataLl2ThresholdSpec>;
#[doc = "Register `CFG_DATA_LL2_THRESHOLD` writer"]
pub type W = crate::W<CfgDataLl2ThresholdSpec>;
#[doc = "Field `LL2_RD_THRESHOLD` reader - 6:0\\]
Configure the CBUFF Read threshold to be Reached before sending the data over CSI2/LVDS and start draining the CBUFF FIFO. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
pub type Ll2RdThresholdR = crate::FieldReader;
#[doc = "Field `LL2_RD_THRESHOLD` writer - 6:0\\]
Configure the CBUFF Read threshold to be Reached before sending the data over CSI2/LVDS and start draining the CBUFF FIFO. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
pub type Ll2RdThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LL2_WR_THRESHOLD` reader - 14:8\\]
Configure the CBUFF FIFO Write threshold over which CBUFF will stall the DMA write to the CBUFF. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
pub type Ll2WrThresholdR = crate::FieldReader;
#[doc = "Field `LL2_WR_THRESHOLD` writer - 14:8\\]
Configure the CBUFF FIFO Write threshold over which CBUFF will stall the DMA write to the CBUFF. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
pub type Ll2WrThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::BitReader;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ll2dman` reader - 18:16\\]
If the long Packet Header is enabled, CBUFF can generate a DMA request to trigger the DMA trasnfer for the new packet 0 : Send a Request on DMA HW Req output line 0 1 : Send a Request on DMA HW Req output line 1 2 : Send a Request on DMA HW Req output line 2 3 : Send a Request on DMA HW Req output line 3 4 : Send a Request on DMA HW Req output line 4 5 : Send a Request on DMA HW Req output line 5 6 : Send a Request on DMA HW Req output line 6 7 : Do not generate dma trigger"]
pub type Ll2dmanR = crate::FieldReader;
#[doc = "Field `ll2dman` writer - 18:16\\]
If the long Packet Header is enabled, CBUFF can generate a DMA request to trigger the DMA trasnfer for the new packet 0 : Send a Request on DMA HW Req output line 0 1 : Send a Request on DMA HW Req output line 1 2 : Send a Request on DMA HW Req output line 2 3 : Send a Request on DMA HW Req output line 3 4 : Send a Request on DMA HW Req output line 4 5 : Send a Request on DMA HW Req output line 5 6 : Send a Request on DMA HW Req output line 6 7 : Do not generate dma trigger"]
pub type Ll2dmanW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU3` reader - "]
pub type Nu3R = crate::FieldReader<u16>;
#[doc = "Field `NU3` writer - "]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Configure the CBUFF Read threshold to be Reached before sending the data over CSI2/LVDS and start draining the CBUFF FIFO. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
    #[inline(always)]
    pub fn ll2_rd_threshold(&self) -> Ll2RdThresholdR {
        Ll2RdThresholdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Configure the CBUFF FIFO Write threshold over which CBUFF will stall the DMA write to the CBUFF. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
    #[inline(always)]
    pub fn ll2_wr_threshold(&self) -> Ll2WrThresholdR {
        Ll2WrThresholdR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
If the long Packet Header is enabled, CBUFF can generate a DMA request to trigger the DMA trasnfer for the new packet 0 : Send a Request on DMA HW Req output line 0 1 : Send a Request on DMA HW Req output line 1 2 : Send a Request on DMA HW Req output line 2 3 : Send a Request on DMA HW Req output line 3 4 : Send a Request on DMA HW Req output line 4 5 : Send a Request on DMA HW Req output line 5 6 : Send a Request on DMA HW Req output line 6 7 : Do not generate dma trigger"]
    #[inline(always)]
    pub fn ll2dman(&self) -> Ll2dmanR {
        Ll2dmanR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Configure the CBUFF Read threshold to be Reached before sending the data over CSI2/LVDS and start draining the CBUFF FIFO. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
    #[inline(always)]
    #[must_use]
    pub fn ll2_rd_threshold(&mut self) -> Ll2RdThresholdW<CfgDataLl2ThresholdSpec> {
        Ll2RdThresholdW::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<CfgDataLl2ThresholdSpec> {
        Nu1W::new(self, 7)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Configure the CBUFF FIFO Write threshold over which CBUFF will stall the DMA write to the CBUFF. Static configuration. This can be programmed to fixed value mentioned in the Programming Model"]
    #[inline(always)]
    #[must_use]
    pub fn ll2_wr_threshold(&mut self) -> Ll2WrThresholdW<CfgDataLl2ThresholdSpec> {
        Ll2WrThresholdW::new(self, 8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<CfgDataLl2ThresholdSpec> {
        Nu2W::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
If the long Packet Header is enabled, CBUFF can generate a DMA request to trigger the DMA trasnfer for the new packet 0 : Send a Request on DMA HW Req output line 0 1 : Send a Request on DMA HW Req output line 1 2 : Send a Request on DMA HW Req output line 2 3 : Send a Request on DMA HW Req output line 3 4 : Send a Request on DMA HW Req output line 4 5 : Send a Request on DMA HW Req output line 5 6 : Send a Request on DMA HW Req output line 6 7 : Do not generate dma trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ll2dman(&mut self) -> Ll2dmanW<CfgDataLl2ThresholdSpec> {
        Ll2dmanW::new(self, 16)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<CfgDataLl2ThresholdSpec> {
        Nu3W::new(self, 19)
    }
}
#[doc = "CFG_DATA_LL2_THRESHOLD\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll2_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll2_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDataLl2ThresholdSpec;
impl crate::RegisterSpec for CfgDataLl2ThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data_ll2_threshold::R`](R) reader structure"]
impl crate::Readable for CfgDataLl2ThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_data_ll2_threshold::W`](W) writer structure"]
impl crate::Writable for CfgDataLl2ThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DATA_LL2_THRESHOLD to value 0"]
impl crate::Resettable for CfgDataLl2ThresholdSpec {
    const RESET_VALUE: u32 = 0;
}
