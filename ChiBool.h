/*
 * ChiBool.h
 *
 *  Created on: 2015年11月8日
 *      Author: Mickey
 */

#ifndef CHIBOOL_H_
#define CHIBOOL_H_

#include "ChiObj.h"

class ChiBool: public ChiObj {
	bool _b;
public:
	ChiBool();
	virtual ~ChiBool();
};

#endif /* CHIBOOL_H_ */
