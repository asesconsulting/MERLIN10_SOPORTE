<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TributaryCond_VA</name>
   <tag></tag>
   <elementGuidId>ddb4b9dd-4a8e-4942-9256-a4c73b927a42</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.condtribonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:queryCondtrib>
         &lt;!--Optional:-->
         &lt;name>
            &lt;!--Optional:-->
            &lt;cuit>30707171282&lt;/cuit>
            &lt;!--Optional:-->
            &lt;denominacion> &lt;/denominacion>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;sector> &lt;/sector>
            &lt;!--Optional:-->
            &lt;userName> &lt;/userName>
         &lt;/name>
      &lt;/soap:queryCondtrib>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>queryCondtrib</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Tributaria</defaultValue>
      <description></description>
      <id>b8665ba3-061b-46fe-bbcf-90179a343b75</id>
      <masked>false</masked>
      <name>Tributaria</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




WS.verifyElementText(response, 'queryCondtribResponse.return.codigoActividad', '00')
WS.verifyElementText(response, 'queryCondtribResponse.return.cuit', '30707171282')
WS.verifyElementText(response, 'queryCondtribResponse.return.cuitInfo', '30707171282')
WS.verifyElementText(response, 'queryCondtribResponse.return.denominacion', 'ODFJELL ARGENTINA S.A.        ')
WS.verifyElementText(response, 'queryCondtribResponse.return.denominacionInfo', '')
WS.verifyElementText(response, 'queryCondtribResponse.return.empleador', 'S')
WS.verifyElementText(response, 'queryCondtribResponse.return.estado', 'VA')
WS.verifyElementText(response, 'queryCondtribResponse.return.flagDenominacion', 'NN')
WS.verifyElementText(response, 'queryCondtribResponse.return.ganancias', 'AC')
WS.verifyElementText(response, 'queryCondtribResponse.return.iva', 'AC')
WS.verifyElementText(response, 'queryCondtribResponse.return.monotributo', 'NI')
WS.verifyElementText(response, 'queryCondtribResponse.return.sociedad', 'N')</verificationScript>
   <wsdlAddress>${Tributaria}</wsdlAddress>
</WebServiceRequestEntity>
