/*
 * ChiRef.h
 *
 *  Created on: 2015Äê11ÔÂ1ÈÕ
 *      Author: Mickey
 */

#ifndef CHIREF_H_
#define CHIREF_H_

#include <memory>

class ChiRef {
	std::shared_ptr<void> p;
public:
	ChiRef(void * pi);
	virtual ~ChiRef();
};

#endif /* CHIREF_H_ */
