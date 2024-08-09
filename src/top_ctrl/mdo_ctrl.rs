#[doc = "Register `MDO_CTRL` reader"]
pub type R = crate::R<MdoCtrlSpec>;
#[doc = "Register `MDO_CTRL` writer"]
pub type W = crate::W<MdoCtrlSpec>;
#[doc = "Field `AURORATX_SRC_SELECT` reader - 0:0\\]
Select the TPIU source to TOP_AURORATX IP 0:Measurement Data 1: Trace Data"]
pub type AuroratxSrcSelectR = crate::BitReader;
#[doc = "Field `AURORATX_SRC_SELECT` writer - 0:0\\]
Select the TPIU source to TOP_AURORATX IP 0:Measurement Data 1: Trace Data"]
pub type AuroratxSrcSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_SELECT` reader - 5:4\\]
Select the source IP of LVDS Data 0: Aurora full data on LVDS 1: CBUFF on LVDS 2: Aurora bit-clk ,frame clk and 2 data support. 3: Aurora 1bit clk and 2 data support for LOP package"]
pub type SrcSelectR = crate::FieldReader;
#[doc = "Field `SRC_SELECT` writer - 5:4\\]
Select the source IP of LVDS Data 0: Aurora full data on LVDS 1: CBUFF on LVDS 2: Aurora bit-clk ,frame clk and 2 data support. 3: Aurora 1bit clk and 2 data support for LOP package"]
pub type SrcSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select the TPIU source to TOP_AURORATX IP 0:Measurement Data 1: Trace Data"]
    #[inline(always)]
    pub fn auroratx_src_select(&self) -> AuroratxSrcSelectR {
        AuroratxSrcSelectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select the source IP of LVDS Data 0: Aurora full data on LVDS 1: CBUFF on LVDS 2: Aurora bit-clk ,frame clk and 2 data support. 3: Aurora 1bit clk and 2 data support for LOP package"]
    #[inline(always)]
    pub fn src_select(&self) -> SrcSelectR {
        SrcSelectR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select the TPIU source to TOP_AURORATX IP 0:Measurement Data 1: Trace Data"]
    #[inline(always)]
    #[must_use]
    pub fn auroratx_src_select(&mut self) -> AuroratxSrcSelectW<MdoCtrlSpec> {
        AuroratxSrcSelectW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select the source IP of LVDS Data 0: Aurora full data on LVDS 1: CBUFF on LVDS 2: Aurora bit-clk ,frame clk and 2 data support. 3: Aurora 1bit clk and 2 data support for LOP package"]
    #[inline(always)]
    #[must_use]
    pub fn src_select(&mut self) -> SrcSelectW<MdoCtrlSpec> {
        SrcSelectW::new(self, 4)
    }
}
#[doc = "MDO_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mdo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdoCtrlSpec;
impl crate::RegisterSpec for MdoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdo_ctrl::R`](R) reader structure"]
impl crate::Readable for MdoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mdo_ctrl::W`](W) writer structure"]
impl crate::Writable for MdoCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDO_CTRL to value 0"]
impl crate::Resettable for MdoCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
