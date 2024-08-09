#[doc = "Register `SEG0_START_ADDR` reader"]
pub type R = crate::R<Seg0StartAddrSpec>;
#[doc = "Register `SEG0_START_ADDR` writer"]
pub type W = crate::W<Seg0StartAddrSpec>;
#[doc = "Field `SEG_START_ADDR` reader - 19:0\\]
Segment 0 Start Address (RWP - Read, Priviledge Mode Write only) This register holds the ROM address for the start of first interval of the segment. When STC_GCR0.RS_CNT_B1 field is set to (1x) ΓÇ£PRELOADΓÇ¥ option, this register is used to determine the ROM start address for the Segment selected in ST_SEGPLR register. Valid number of bits depends on RTL paramerter ADDR"]
pub type SegStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `SEG_START_ADDR` writer - 19:0\\]
Segment 0 Start Address (RWP - Read, Priviledge Mode Write only) This register holds the ROM address for the start of first interval of the segment. When STC_GCR0.RS_CNT_B1 field is set to (1x) ΓÇ£PRELOADΓÇ¥ option, this register is used to determine the ROM start address for the Segment selected in ST_SEGPLR register. Valid number of bits depends on RTL paramerter ADDR"]
pub type SegStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `NU13` reader - 31:20\\]
Reserved bits"]
pub type Nu13R = crate::FieldReader<u16>;
#[doc = "Field `NU13` writer - 31:20\\]
Reserved bits"]
pub type Nu13W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Segment 0 Start Address (RWP - Read, Priviledge Mode Write only) This register holds the ROM address for the start of first interval of the segment. When STC_GCR0.RS_CNT_B1 field is set to (1x) ΓÇ£PRELOADΓÇ¥ option, this register is used to determine the ROM start address for the Segment selected in ST_SEGPLR register. Valid number of bits depends on RTL paramerter ADDR"]
    #[inline(always)]
    pub fn seg_start_addr(&self) -> SegStartAddrR {
        SegStartAddrR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu13(&self) -> Nu13R {
        Nu13R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Segment 0 Start Address (RWP - Read, Priviledge Mode Write only) This register holds the ROM address for the start of first interval of the segment. When STC_GCR0.RS_CNT_B1 field is set to (1x) ΓÇ£PRELOADΓÇ¥ option, this register is used to determine the ROM start address for the Segment selected in ST_SEGPLR register. Valid number of bits depends on RTL paramerter ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn seg_start_addr(&mut self) -> SegStartAddrW<Seg0StartAddrSpec> {
        SegStartAddrW::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu13(&mut self) -> Nu13W<Seg0StartAddrSpec> {
        Nu13W::new(self, 20)
    }
}
#[doc = "ROM Start address for Segment0\n\nYou can [`read`](crate::Reg::read) this register and get [`seg0_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg0_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seg0StartAddrSpec;
impl crate::RegisterSpec for Seg0StartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seg0_start_addr::R`](R) reader structure"]
impl crate::Readable for Seg0StartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`seg0_start_addr::W`](W) writer structure"]
impl crate::Writable for Seg0StartAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEG0_START_ADDR to value 0"]
impl crate::Resettable for Seg0StartAddrSpec {
    const RESET_VALUE: u32 = 0;
}
