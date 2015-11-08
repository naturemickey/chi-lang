/*
 * ChiInt.h
 *
 *  Created on: 2015年11月8日
 *      Author: Mickey
 */

#ifndef CHIINT_H_
#define CHIINT_H_

#include "ChiObj.h"

class ChiInt: public ChiObj {
	/**
	 * The signum of this ChiInt: -1 for negative, 0 for zero, or
	 * 1 for positive.  Note that the ChiInt zero <i>must</i> have
	 * a signum of 0.  This is necessary to ensures that there is exactly one
	 * representation for each ChiInt value.
	 */
	int signum;
	/**
	 * The magnitude of this BigInteger, in <i>big-endian</i> order: the
	 * zeroth element of this array is the most-significant int of the
	 * magnitude.  The magnitude must be "minimal" in that the most-significant
	 * int ({@code mag[0]}) must be non-zero.  This is necessary to
	 * ensure that there is exactly one representation for each BigInteger
	 * value.  Note that this implies that the BigInteger zero has a
	 * zero-length mag array.
	 */
	int mag[];
public:
	ChiInt();
	virtual ~ChiInt();
};

#endif /* CHIINT_H_ */
