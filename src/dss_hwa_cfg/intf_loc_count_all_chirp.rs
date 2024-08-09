#[doc = "Register `INTF_LOC_COUNT_ALL_CHIRP` reader"]
pub type R = crate::R<IntfLocCountAllChirpSpec>;
#[doc = "Register `INTF_LOC_COUNT_ALL_CHIRP` writer"]
pub type W = crate::W<IntfLocCountAllChirpSpec>;
#[doc = "Field `intf_loc_count_all_chirp` reader - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
pub type IntfLocCountAllChirpR = crate::FieldReader<u16>;
#[doc = "Field `intf_loc_count_all_chirp` writer - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
pub type IntfLocCountAllChirpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
    #[inline(always)]
    pub fn intf_loc_count_all_chirp(&self) -> IntfLocCountAllChirpR {
        IntfLocCountAllChirpR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
    #[inline(always)]
    #[must_use]
    pub fn intf_loc_count_all_chirp(&mut self) -> IntfLocCountAllChirpW<IntfLocCountAllChirpSpec> {
        IntfLocCountAllChirpW::new(self, 0)
    }
}
#[doc = "INTF_LOC_COUNT_ALL_CHIRP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_loc_count_all_chirp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_loc_count_all_chirp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfLocCountAllChirpSpec;
impl crate::RegisterSpec for IntfLocCountAllChirpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_loc_count_all_chirp::R`](R) reader structure"]
impl crate::Readable for IntfLocCountAllChirpSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_loc_count_all_chirp::W`](W) writer structure"]
impl crate::Writable for IntfLocCountAllChirpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_LOC_COUNT_ALL_CHIRP to value 0"]
impl crate::Resettable for IntfLocCountAllChirpSpec {
    const RESET_VALUE: u32 = 0;
}
