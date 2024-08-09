#[doc = "Register `CPSW_NC_EST_FETCH_LOC_112` reader"]
pub type R = crate::R<CpswNcEstFetchLoc112Spec>;
#[doc = "Register `CPSW_NC_EST_FETCH_LOC_112` writer"]
pub type W = crate::W<CpswNcEstFetchLoc112Spec>;
#[doc = "Field `RAM_LOCATION` reader - 21:0\\]
RAM Location"]
pub type RamLocationR = crate::FieldReader<u32>;
#[doc = "Field `RAM_LOCATION` writer - 21:0\\]
RAM Location"]
pub type RamLocationW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - 21:0\\]
RAM Location"]
    #[inline(always)]
    pub fn ram_location(&self) -> RamLocationR {
        RamLocationR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - 21:0\\]
RAM Location"]
    #[inline(always)]
    #[must_use]
    pub fn ram_location(&mut self) -> RamLocationW<CpswNcEstFetchLoc112Spec> {
        RamLocationW::new(self, 0)
    }
}
#[doc = "The Revision Register contains the ID and revision information.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_fetch_loc_112::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_fetch_loc_112::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEstFetchLoc112Spec;
impl crate::RegisterSpec for CpswNcEstFetchLoc112Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_est_fetch_loc_112::R`](R) reader structure"]
impl crate::Readable for CpswNcEstFetchLoc112Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_est_fetch_loc_112::W`](W) writer structure"]
impl crate::Writable for CpswNcEstFetchLoc112Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_EST_FETCH_LOC_112 to value 0"]
impl crate::Resettable for CpswNcEstFetchLoc112Spec {
    const RESET_VALUE: u32 = 0;
}
