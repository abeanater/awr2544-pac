#[doc = "Register `vector` reader"]
pub type R = crate::R<VectorSpec>;
#[doc = "Register `vector` writer"]
pub type W = crate::W<VectorSpec>;
#[doc = "Field `VALUE_WRITTEN_TO` reader - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
pub type ValueWrittenToR = crate::FieldReader<u16>;
#[doc = "Field `VALUE_WRITTEN_TO` writer - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
pub type ValueWrittenToW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `WRITE_1_TO` reader - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
pub type Write1ToR = crate::BitReader;
#[doc = "Field `WRITE_1_TO` writer - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
pub type Write1ToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_ADDRESS` reader - 23:16\\]
Read address"]
pub type ReadAddressR = crate::FieldReader;
#[doc = "Field `READ_ADDRESS` writer - 23:16\\]
Read address"]
pub type ReadAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STATUS_TO_INDICATE` reader - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
pub type StatusToIndicateR = crate::BitReader;
#[doc = "Field `STATUS_TO_INDICATE` writer - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
pub type StatusToIndicateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
    #[inline(always)]
    pub fn value_written_to(&self) -> ValueWrittenToR {
        ValueWrittenToR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
    #[inline(always)]
    pub fn write_1_to(&self) -> Write1ToR {
        Write1ToR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address"]
    #[inline(always)]
    pub fn read_address(&self) -> ReadAddressR {
        ReadAddressR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
    #[inline(always)]
    pub fn status_to_indicate(&self) -> StatusToIndicateR {
        StatusToIndicateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
    #[inline(always)]
    #[must_use]
    pub fn value_written_to(&mut self) -> ValueWrittenToW<VectorSpec> {
        ValueWrittenToW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS"]
    #[inline(always)]
    #[must_use]
    pub fn write_1_to(&mut self) -> Write1ToW<VectorSpec> {
        Write1ToW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address"]
    #[inline(always)]
    #[must_use]
    pub fn read_address(&mut self) -> ReadAddressW<VectorSpec> {
        ReadAddressW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn status_to_indicate(&mut self) -> StatusToIndicateW<VectorSpec> {
        StatusToIndicateW::new(self, 24)
    }
}
#[doc = "ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vector::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vector::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VectorSpec;
impl crate::RegisterSpec for VectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vector::R`](R) reader structure"]
impl crate::Readable for VectorSpec {}
#[doc = "`write(|w| ..)` method takes [`vector::W`](W) writer structure"]
impl crate::Writable for VectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vector to value 0"]
impl crate::Resettable for VectorSpec {
    const RESET_VALUE: u32 = 0;
}
