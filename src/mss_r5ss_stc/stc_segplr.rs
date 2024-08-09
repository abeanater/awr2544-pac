#[doc = "Register `STC_SEGPLR` reader"]
pub type R = crate::R<StcSegplrSpec>;
#[doc = "Register `STC_SEGPLR` writer"]
pub type W = crate::W<StcSegplrSpec>;
#[doc = "Field `SEGID_PLOAD` reader - 1:0\\]
Segment number for which preload is to be started (RWP - Read, Priviledge Mode Write only) This specifies the segment for which the address of its First interval will be pre-loaded into the NSTC ROM address counter. The 1st address of each segment are defined in SEGx_START_ADDR register.The address of the 1st interval of the selected segment is loaded into the NSTC ROM address counter when the RS_CNT_B1 bits of STC_GCR0 are set to 1X 00 = Preload the address of the 1st interval of segment 0. 01 = Preload the address of the 1st interval of segment 1. 10 = Preload the address of the 1st interval of segment 2. 11 = Preload the address of the 1st interval of segment 3."]
pub type SegidPloadR = crate::FieldReader;
#[doc = "Field `SEGID_PLOAD` writer - 1:0\\]
Segment number for which preload is to be started (RWP - Read, Priviledge Mode Write only) This specifies the segment for which the address of its First interval will be pre-loaded into the NSTC ROM address counter. The 1st address of each segment are defined in SEGx_START_ADDR register.The address of the 1st interval of the selected segment is loaded into the NSTC ROM address counter when the RS_CNT_B1 bits of STC_GCR0 are set to 1X 00 = Preload the address of the 1st interval of segment 0. 01 = Preload the address of the 1st interval of segment 1. 10 = Preload the address of the 1st interval of segment 2. 11 = Preload the address of the 1st interval of segment 3."]
pub type SegidPloadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU12` reader - 31:2\\]
Reserved bits"]
pub type Nu12R = crate::FieldReader<u32>;
#[doc = "Field `NU12` writer - 31:2\\]
Reserved bits"]
pub type Nu12W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Segment number for which preload is to be started (RWP - Read, Priviledge Mode Write only) This specifies the segment for which the address of its First interval will be pre-loaded into the NSTC ROM address counter. The 1st address of each segment are defined in SEGx_START_ADDR register.The address of the 1st interval of the selected segment is loaded into the NSTC ROM address counter when the RS_CNT_B1 bits of STC_GCR0 are set to 1X 00 = Preload the address of the 1st interval of segment 0. 01 = Preload the address of the 1st interval of segment 1. 10 = Preload the address of the 1st interval of segment 2. 11 = Preload the address of the 1st interval of segment 3."]
    #[inline(always)]
    pub fn segid_pload(&self) -> SegidPloadR {
        SegidPloadR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu12(&self) -> Nu12R {
        Nu12R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Segment number for which preload is to be started (RWP - Read, Priviledge Mode Write only) This specifies the segment for which the address of its First interval will be pre-loaded into the NSTC ROM address counter. The 1st address of each segment are defined in SEGx_START_ADDR register.The address of the 1st interval of the selected segment is loaded into the NSTC ROM address counter when the RS_CNT_B1 bits of STC_GCR0 are set to 1X 00 = Preload the address of the 1st interval of segment 0. 01 = Preload the address of the 1st interval of segment 1. 10 = Preload the address of the 1st interval of segment 2. 11 = Preload the address of the 1st interval of segment 3."]
    #[inline(always)]
    #[must_use]
    pub fn segid_pload(&mut self) -> SegidPloadW<StcSegplrSpec> {
        SegidPloadW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu12(&mut self) -> Nu12W<StcSegplrSpec> {
        Nu12W::new(self, 2)
    }
}
#[doc = "Segment 1st interval Preload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_segplr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_segplr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcSegplrSpec;
impl crate::RegisterSpec for StcSegplrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stc_segplr::R`](R) reader structure"]
impl crate::Readable for StcSegplrSpec {}
#[doc = "`write(|w| ..)` method takes [`stc_segplr::W`](W) writer structure"]
impl crate::Writable for StcSegplrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STC_SEGPLR to value 0"]
impl crate::Resettable for StcSegplrSpec {
    const RESET_VALUE: u32 = 0;
}
