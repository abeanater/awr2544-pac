#[doc = "Register `GPADC_CTRL` reader"]
pub type R = crate::R<GpadcCtrlSpec>;
#[doc = "Register `GPADC_CTRL` writer"]
pub type W = crate::W<GpadcCtrlSpec>;
#[doc = "Field `gpadc_sw_trig` reader - 0:0\\]
Writing 1'b1 will give MMR based SW trigger to GPADC"]
pub type GpadcSwTrigR = crate::BitReader;
#[doc = "Field `gpadc_sw_trig` writer - 0:0\\]
Writing 1'b1 will give MMR based SW trigger to GPADC"]
pub type GpadcSwTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_trigin_sel` reader - 12:8\\]
Writing below decimal values to this regiter will select corresponding interrupt as GPADC trigger source. 0: GPIO_0 1: GPIO_1 2: GPIO_2 3: GPIO_3 4: Reserved 5: Reserved 6: Reserved 7: Reserved 8: Reserved 9: Reseved 10: Reserved 11: MSS_RTIA_INT0 12: MSS_RTIA_INT1 13: HW_Sync_FE1 14: Reserved 15: MMR based SW trigger"]
pub type GpadcTriginSelR = crate::FieldReader;
#[doc = "Field `gpadc_trigin_sel` writer - 12:8\\]
Writing below decimal values to this regiter will select corresponding interrupt as GPADC trigger source. 0: GPIO_0 1: GPIO_1 2: GPIO_2 3: GPIO_3 4: Reserved 5: Reserved 6: Reserved 7: Reserved 8: Reserved 9: Reseved 10: Reserved 11: MSS_RTIA_INT0 12: MSS_RTIA_INT1 13: HW_Sync_FE1 14: Reserved 15: MMR based SW trigger"]
pub type GpadcTriginSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 1'b1 will give MMR based SW trigger to GPADC"]
    #[inline(always)]
    pub fn gpadc_sw_trig(&self) -> GpadcSwTrigR {
        GpadcSwTrigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Writing below decimal values to this regiter will select corresponding interrupt as GPADC trigger source. 0: GPIO_0 1: GPIO_1 2: GPIO_2 3: GPIO_3 4: Reserved 5: Reserved 6: Reserved 7: Reserved 8: Reserved 9: Reseved 10: Reserved 11: MSS_RTIA_INT0 12: MSS_RTIA_INT1 13: HW_Sync_FE1 14: Reserved 15: MMR based SW trigger"]
    #[inline(always)]
    pub fn gpadc_trigin_sel(&self) -> GpadcTriginSelR {
        GpadcTriginSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 1'b1 will give MMR based SW trigger to GPADC"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_sw_trig(&mut self) -> GpadcSwTrigW<GpadcCtrlSpec> {
        GpadcSwTrigW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Writing below decimal values to this regiter will select corresponding interrupt as GPADC trigger source. 0: GPIO_0 1: GPIO_1 2: GPIO_2 3: GPIO_3 4: Reserved 5: Reserved 6: Reserved 7: Reserved 8: Reserved 9: Reseved 10: Reserved 11: MSS_RTIA_INT0 12: MSS_RTIA_INT1 13: HW_Sync_FE1 14: Reserved 15: MMR based SW trigger"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_trigin_sel(&mut self) -> GpadcTriginSelW<GpadcCtrlSpec> {
        GpadcTriginSelW::new(self, 8)
    }
}
#[doc = "GPADC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpadcCtrlSpec;
impl crate::RegisterSpec for GpadcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_ctrl::R`](R) reader structure"]
impl crate::Readable for GpadcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpadc_ctrl::W`](W) writer structure"]
impl crate::Writable for GpadcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPADC_CTRL to value 0"]
impl crate::Resettable for GpadcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
