# Calculating Digits

## Unique Segments

Digits with unique segment counts are

- 1, with 2 segments
- 4, with 4 segments,
- 7, with 3 segments
- 8, with 7 segments

## Determining individual segments

### Phase I: Determining Segment A

First, we can consider the digits 1 and 7. These have unique character counts of 2 and 3, and as such are very easily identifiable. They also both occupy segments C and F. From this, we can determine that the segment character which 7 contains, but 1 does not, must be segment A.

### Phase II: Determining Segment G

Once we know segment A, we can add it to a 4 to have a pseudo-digit containing segments ABCDF. This is one segment short of the segments required to construct a 9; ABCDFG. Although there are other 6 character digits, namely 0 and 6, only 9 shares all 4 of 4's segments. As such, the 6 character segment which contains 4's characters **and\*** the top segment must be a 9. From here, it is trivial to remove our pseudo-digit's ABCDF from the 9's ABCDFG to be left with segment G.

### Phase III-A: Determining Segment E

Once we know the 6 characters constituing a 9, we can simply remove them from the 7 character input which we know must be the 8 to determine segment E.

### Phase III-B: Determining Segment D

Now that we have the top and bottom segments, we can create another pseudo-digit by adding them to 1. This pseudo-digit contains the characters ACFG. A 3 is a 5 segment digit represented by ACDFG. The other 5 segment digits, 2 and 5, are missing characters C and F respectively, so we know that any 5 segment digit containing ACFG must be a 3, and that by process of elimination its 5th character must be our middle segment character.

### Phase IV-A: Determining Segment B

Now that we have the middle segment, we can create a pseudo-digit by adding it to 1. The segment which 4 contains but our pseudo-digit does not must be B.

### Phase IV-B: Determining Segment C

We know that 2 is the only 5 character digit containing segment E. A 2 is comprised of ACDEG, and we now know ADEG. as such, the character sequence containing AxDEG must in face be ACDEG, as it can only be a 2. This gives us character C.

### Phase IV-C: Determining Segment F

We know that 5 is the only 5 character digit containing segment B. A 5 is comprised of ABDFG, and we now know ABDG. as such, the character sequence containing ABDxG must in face be ABDFG, as it can only be a 5. This gives us character F.

Any individual part of phase IV could also be skipped in favour of process of elimination, as once we have 6 characters determined we know the 7th is simply the reamining character.
