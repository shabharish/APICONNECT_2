<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SampleRequest</name>
   <tag></tag>
   <elementGuidId>01432f2d-4402-4ef3-b881-7eb80e995f0c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
&lt;soapenv:Header>
&lt;/soapenv:Header>
&lt;soapenv:Body>
&lt;tns:Add xmlns:tns=&quot;http://tempuri.org/&quot;>&lt;!-- mandatory -->
&lt;tns:intA>3&lt;/tns:intA>
&lt;tns:intB>3&lt;/tns:intB>
&lt;/tns:Add>
&lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Add</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
</verificationScript>
   <wsdlAddress>https://api.eu.apiconnect.ibmcloud.com/shabharishmurthythbscom-dev/test-env/Calculator?wsdl</wsdlAddress>
</WebServiceRequestEntity>
