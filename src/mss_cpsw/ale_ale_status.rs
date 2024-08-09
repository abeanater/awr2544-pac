#[doc = "Register `ALE_ALE_STATUS` reader"]
pub type R = crate::R<AleAleStatusSpec>;
#[doc = "Register `ALE_ALE_STATUS` writer"]
pub type W = crate::W<AleAleStatusSpec>;
#[doc = "Field `THIS_IS_THE` reader - 4:0\\]
This is the number of table entries total divided by 1024. A value of 1 indicates 1024 table entries. A value of 8 indicates 8192 table entries."]
pub type ThisIsTheR = crate::FieldReader;
#[doc = "Field `THIS_IS_THE` writer - 4:0\\]
This is the number of table entries total divided by 1024. A value of 1 indicates 1024 table entries. A value of 8 indicates 8192 table entries."]
pub type ThisIsTheW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `THE_NUMBER_OF` reader - 6:6\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 32 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
pub type TheNumberOfR = crate::BitReader;
#[doc = "Field `THE_NUMBER_OF` writer - 6:6\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 32 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
pub type TheNumberOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THE_NUMBER_OF` reader - 7:7\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 128 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
pub type TheNumberOfR = crate::BitReader;
#[doc = "Field `THE_NUMBER_OF` writer - 7:7\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 128 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
pub type TheNumberOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THIS_IS_THE` reader - 15:8\\]
This is the number of Classifiers the ALE implements divided by 8. A value of 4 indicates 32 policer engines total."]
pub type ThisIsTheR = crate::FieldReader;
#[doc = "Field `THIS_IS_THE` writer - 15:8\\]
This is the number of Classifiers the ALE implements divided by 8. A value of 4 indicates 32 policer engines total."]
pub type ThisIsTheW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WHEN_SET__THE` reader - 30:30\\]
When set, the unregistered multicast field is a mask versus an index on 8 bit boundary in the ALE table."]
pub type WhenSet_TheR = crate::BitReader;
#[doc = "Field `WHEN_SET__THE` writer - 30:30\\]
When set, the unregistered multicast field is a mask versus an index on 8 bit boundary in the ALE table."]
pub type WhenSet_TheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHEN_SET__THE` reader - 31:31\\]
When set, the unregistered multicast field is a mask versus an index on 12 bit boundary in the ALE table."]
pub type WhenSet_TheR = crate::BitReader;
#[doc = "Field `WHEN_SET__THE` writer - 31:31\\]
When set, the unregistered multicast field is a mask versus an index on 12 bit boundary in the ALE table."]
pub type WhenSet_TheW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
This is the number of table entries total divided by 1024. A value of 1 indicates 1024 table entries. A value of 8 indicates 8192 table entries."]
    #[inline(always)]
    pub fn this_is_the(&self) -> ThisIsTheR {
        ThisIsTheR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 32 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
    #[inline(always)]
    pub fn the_number_of(&self) -> TheNumberOfR {
        TheNumberOfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 128 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
    #[inline(always)]
    pub fn the_number_of(&self) -> TheNumberOfR {
        TheNumberOfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This is the number of Classifiers the ALE implements divided by 8. A value of 4 indicates 32 policer engines total."]
    #[inline(always)]
    pub fn this_is_the(&self) -> ThisIsTheR {
        ThisIsTheR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
When set, the unregistered multicast field is a mask versus an index on 8 bit boundary in the ALE table."]
    #[inline(always)]
    pub fn when_set__the(&self) -> WhenSet_TheR {
        WhenSet_TheR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, the unregistered multicast field is a mask versus an index on 12 bit boundary in the ALE table."]
    #[inline(always)]
    pub fn when_set__the(&self) -> WhenSet_TheR {
        WhenSet_TheR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
This is the number of table entries total divided by 1024. A value of 1 indicates 1024 table entries. A value of 8 indicates 8192 table entries."]
    #[inline(always)]
    #[must_use]
    pub fn this_is_the(&mut self) -> ThisIsTheW<AleAleStatusSpec> {
        ThisIsTheW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 32 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
    #[inline(always)]
    #[must_use]
    pub fn the_number_of(&mut self) -> TheNumberOfW<AleAleStatusSpec> {
        TheNumberOfW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
The number of ALE entries per slice of the table when this is set it indicates the depth is 128 if both ramdepth128 and ramdepth32 are zero the depth is 64."]
    #[inline(always)]
    #[must_use]
    pub fn the_number_of(&mut self) -> TheNumberOfW<AleAleStatusSpec> {
        TheNumberOfW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This is the number of Classifiers the ALE implements divided by 8. A value of 4 indicates 32 policer engines total."]
    #[inline(always)]
    #[must_use]
    pub fn this_is_the(&mut self) -> ThisIsTheW<AleAleStatusSpec> {
        ThisIsTheW::new(self, 8)
    }
    #[doc = "Bit 30 - 30:30\\]
When set, the unregistered multicast field is a mask versus an index on 8 bit boundary in the ALE table."]
    #[inline(always)]
    #[must_use]
    pub fn when_set__the(&mut self) -> WhenSet_TheW<AleAleStatusSpec> {
        WhenSet_TheW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, the unregistered multicast field is a mask versus an index on 12 bit boundary in the ALE table."]
    #[inline(always)]
    #[must_use]
    pub fn when_set__the(&mut self) -> WhenSet_TheW<AleAleStatusSpec> {
        WhenSet_TheW::new(self, 31)
    }
}
#[doc = "The ALE status provides information on the ALE configuration and state. The ~iramdepth is used to determine how IPv6 entries are stored in the table. IPv6 entries are stored in two entries where IPv6 Entry hi is designated by the odd slice index and lo is designated by the even slice index. The slice index is above the ram depth like {SlixeIndex,RamIndex}. So for a 64 deep RAM index of 0x005, the Hi portion of the IPv6 entry is located at 0x005|0x040 and the Lo portion is located at 0x005&amp;amp;(~0x040).\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleStatusSpec;
impl crate::RegisterSpec for AleAleStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_status::R`](R) reader structure"]
impl crate::Readable for AleAleStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_status::W`](W) writer structure"]
impl crate::Writable for AleAleStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_STATUS to value 0"]
impl crate::Resettable for AleAleStatusSpec {
    const RESET_VALUE: u32 = 0;
}
