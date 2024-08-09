#[doc = "Register `CPSW_NC_CPPI_P0_FLOW_ID_OFFSET_REG` reader"]
pub type R = crate::R<CpswNcCppiP0FlowIdOffsetRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_FLOW_ID_OFFSET_REG` writer"]
pub type W = crate::W<CpswNcCppiP0FlowIdOffsetRegSpec>;
#[doc = "Field `THIS_VALUE_IS` reader - 13:0\\]
This value is added to the thread/Flow_ID in CPPI transmit PSI Info Word 0"]
pub type ThisValueIsR = crate::FieldReader<u16>;
#[doc = "Field `THIS_VALUE_IS` writer - 13:0\\]
This value is added to the thread/Flow_ID in CPPI transmit PSI Info Word 0"]
pub type ThisValueIsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
This value is added to the thread/Flow_ID in CPPI transmit PSI Info Word 0"]
    #[inline(always)]
    pub fn this_value_is(&self) -> ThisValueIsR {
        ThisValueIsR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
This value is added to the thread/Flow_ID in CPPI transmit PSI Info Word 0"]
    #[inline(always)]
    #[must_use]
    pub fn this_value_is(&mut self) -> ThisValueIsW<CpswNcCppiP0FlowIdOffsetRegSpec> {
        ThisValueIsW::new(self, 0)
    }
}
#[doc = "CPPI Port 0 Flow ID Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_flow_id_offset_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_flow_id_offset_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0FlowIdOffsetRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0FlowIdOffsetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_flow_id_offset_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0FlowIdOffsetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_flow_id_offset_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0FlowIdOffsetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_FLOW_ID_OFFSET_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0FlowIdOffsetRegSpec {
    const RESET_VALUE: u32 = 0;
}
