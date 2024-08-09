#[doc = "Register `CPSW_HW_TRIG_CTRL` reader"]
pub type R = crate::R<CpswHwTrigCtrlSpec>;
#[doc = "Register `CPSW_HW_TRIG_CTRL` writer"]
pub type W = crate::W<CpswHwTrigCtrlSpec>;
#[doc = "Field `CPSW_HW_TRIG_CTRL` reader - 2:0\\]
Bits \\[1:0\\]
are used for Genf select line bits 2'b00 : Genf0 2'b01 : Genf1 2'b10 : Genf2 2'b11 : Not used Bit2 is used to select Genf or CPSW_HW_TRIG_VAL"]
pub type CpswHwTrigCtrlR = crate::FieldReader;
#[doc = "Field `CPSW_HW_TRIG_CTRL` writer - 2:0\\]
Bits \\[1:0\\]
are used for Genf select line bits 2'b00 : Genf0 2'b01 : Genf1 2'b10 : Genf2 2'b11 : Not used Bit2 is used to select Genf or CPSW_HW_TRIG_VAL"]
pub type CpswHwTrigCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Bits \\[1:0\\]
are used for Genf select line bits 2'b00 : Genf0 2'b01 : Genf1 2'b10 : Genf2 2'b11 : Not used Bit2 is used to select Genf or CPSW_HW_TRIG_VAL"]
    #[inline(always)]
    pub fn cpsw_hw_trig_ctrl(&self) -> CpswHwTrigCtrlR {
        CpswHwTrigCtrlR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Bits \\[1:0\\]
are used for Genf select line bits 2'b00 : Genf0 2'b01 : Genf1 2'b10 : Genf2 2'b11 : Not used Bit2 is used to select Genf or CPSW_HW_TRIG_VAL"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw_hw_trig_ctrl(&mut self) -> CpswHwTrigCtrlW<CpswHwTrigCtrlSpec> {
        CpswHwTrigCtrlW::new(self, 0)
    }
}
#[doc = "CPSW_HW_TRIG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_hw_trig_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_hw_trig_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswHwTrigCtrlSpec;
impl crate::RegisterSpec for CpswHwTrigCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_hw_trig_ctrl::R`](R) reader structure"]
impl crate::Readable for CpswHwTrigCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_hw_trig_ctrl::W`](W) writer structure"]
impl crate::Writable for CpswHwTrigCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_HW_TRIG_CTRL to value 0"]
impl crate::Resettable for CpswHwTrigCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
