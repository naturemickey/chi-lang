/*
 * ParamRef.h
 *
 *  Created on: 2015年11月8日
 *      Author: Mickey
 */

#ifndef ARRAY_H_
#define ARRAY_H_

#include <vector>
#include <memory>

class Array {
	std::vector<std::shared_ptr<void>> p;
public:
	Array();
	virtual ~Array();
};

#endif /* ARRAY_H_ */
