#[doc = "Register `PBIDX` reader"]
pub type R = crate::R<PbidxSpec>;
#[doc = "Register `PBIDX` writer"]
pub type W = crate::W<PbidxSpec>;
#[doc = "Field `SOURCE_BIDX_FOR` reader - 15:0\\]
Source B-Idx for Program Register Set:#br#B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\]. SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
pub type SourceBidxForR = crate::FieldReader<u16>;
#[doc = "Field `SOURCE_BIDX_FOR` writer - 15:0\\]
Source B-Idx for Program Register Set:#br#B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\]. SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
pub type SourceBidxForW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DEST_BIDX_FOR` reader - 31:16\\]
Dest B-Idx for Program Register Set:#br#B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\]. DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
pub type DestBidxForR = crate::FieldReader<u16>;
#[doc = "Field `DEST_BIDX_FOR` writer - 31:16\\]
Dest B-Idx for Program Register Set:#br#B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\]. DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
pub type DestBidxForW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Source B-Idx for Program Register Set:#br#B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\]. SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
    #[inline(always)]
    pub fn source_bidx_for(&self) -> SourceBidxForR {
        SourceBidxForR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Dest B-Idx for Program Register Set:#br#B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\]. DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
    #[inline(always)]
    pub fn dest_bidx_for(&self) -> DestBidxForR {
        DestBidxForR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Source B-Idx for Program Register Set:#br#B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\]. SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn source_bidx_for(&mut self) -> SourceBidxForW<PbidxSpec> {
        SourceBidxForW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Dest B-Idx for Program Register Set:#br#B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\]. DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn dest_bidx_for(&mut self) -> DestBidxForW<PbidxSpec> {
        DestBidxForW::new(self, 16)
    }
}
#[doc = "Prog Set B-Dim Idx\n\nYou can [`read`](crate::Reg::read) this register and get [`pbidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbidxSpec;
impl crate::RegisterSpec for PbidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbidx::R`](R) reader structure"]
impl crate::Readable for PbidxSpec {}
#[doc = "`write(|w| ..)` method takes [`pbidx::W`](W) writer structure"]
impl crate::Writable for PbidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIDX to value 0"]
impl crate::Resettable for PbidxSpec {
    const RESET_VALUE: u32 = 0;
}
